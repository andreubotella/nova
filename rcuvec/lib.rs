use std::{alloc::Layout, cell::UnsafeCell, mem, ptr::{write, NonNull}, sync::{atomic::{AtomicPtr, AtomicU32, Ordering}, Arc}};

/// Provides exclusive access to the contained value
/// 
/// The purpose of this type is to split into mutator
/// and reader cells.
#[derive(Debug)]
pub struct ExclusiveCell<T: Sync>(Box<T>);

impl<T> ExclusiveCell<T> where T: Sized + Sync {
    pub fn new(t: T) -> Self {
        Self(Box::new(t))
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.as_mut()
    }

    pub fn split_to_threads(self) -> MutatorCell<T> {
        MutatorCell::new(self.0)
    }
}

#[derive(Debug)]
pub struct MutatorCell<T: Sized + Sync>(*mut T, Arc<*mut T>);

impl<T: Sized + Sync> MutatorCell<T> {
    fn new(t: Box<T>)  -> Self {
        let t_pointer = Box::<T>::leak(t) as *mut T;
        MutatorCell(t_pointer, Arc::new(t_pointer))
    }

    pub fn get_reader(&mut self) -> ReaderCell<T> {
        ReaderCell::new(self.1.clone())
    }
}

// SAFETY: It is safe to change the mutator thread.
unsafe impl<T: Sized + Sync> Send for MutatorCell<T> {}

#[derive(Debug, Clone)]
pub struct ReaderCell<T: Sized + Sync>(Arc<*mut T>);

impl<T: Sized + Sync> ReaderCell<T> {
    fn new(arc: Arc<*mut T>) -> Self {
        Self(arc)
    }

    fn get(&self) -> &T {
        unsafe { &**self.0 }
    }
}

pub struct RCUVec<T: Sized> {
    ptr: AtomicPtr<T>,
    cap: AtomicU32,
    len: AtomicU32,
}

pub struct VecDropContainer<T: Sized> {
    ptr: NonNull<T>,
    cap: usize,
}

pub struct VecDropData<T: Sized>(Box<VecDropContainer<T>>, fn(Box<VecDropContainer<T>>));

fn drop_vec_drop_container<T: Sized>(container: Box<VecDropContainer<T>>) {
    // We should never be deallocating empty allocations.
    debug_assert_ne!(container.cap, 0);
    // Layout::array checks that the number of bytes is <= usize::MAX,
        // but this is redundant since old_layout.size() <= i32::MAX,
        // so the `unwrap` should never fail.
    let layout = Layout::array::<T>(container.cap).unwrap();
    // SAFETY: The layout was allocated by the global allocator using
    // Layout::array<T>(cap).
    unsafe { std::alloc::dealloc(container.ptr.as_ptr() as *mut u8, layout) }
}

impl<T> RCUVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "RCUVec does not support ZSTs");
        RCUVec {
            ptr: AtomicPtr::new(mem::align_of::<T>() as *mut T),
            cap: 0.into(),
            len: 0.into(),
        }
    }

    /// Grow the vector through an exclusive reference
    fn grow_mut(&mut self) {
        // This can't overflow because we ensure self.cap <= i32::MAX.
        let new_cap = if *self.cap.get_mut() == 0 { 1 } else { 2 * *self.cap.get_mut() };

        // Layout::array checks that the number of bytes is <= usize::MAX,
        // but this is redundant since old_layout.size() <= i32::MAX,
        // so the `unwrap` should never fail.
        let new_layout = Layout::array::<T>(new_cap as usize).unwrap();

        // Ensure that the new allocation doesn't exceed `i32::MAX` bytes.
        assert!(new_layout.size() <= i32::MAX as usize, "Allocation too large");

        let new_ptr = if *self.cap.get_mut() == 0 {
            // SAFETY: Layout guarantees that we're properly aligned for T and
            // we've made sure that new_cap is non-zero.
            unsafe { std::alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(*self.cap.get_mut() as usize).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            // SAFETY: old_ptr has been allocated by the global allocator,
            // layout is always the array layout, new size is double the old
            // size, and is checked to be below i32::MAX.
            // We're also guaranteed that no one is reading the old_ptr currently,
            // as we hold an exclusive reference to self.
            unsafe { std::alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        if new_ptr.is_null() {
            // If allocation fails, `new_ptr` will be null, in which case we abort.
            std::alloc::handle_alloc_error(new_layout)
        } else {
            self.ptr = AtomicPtr::new(new_ptr as *mut T);
            self.cap = AtomicU32::new(new_cap);
        }
    }

    /// Grow the RCUVec through a shared reference
    ///
    /// The function returns a drop-container containing a pointer to the data
    /// of the old allocation and drop function to call with the data when drop
    /// is desired. The caller should ensure that no readers to the old pointer
    /// exist when the drop function is called.
    fn grow(&self) -> Option<VecDropData<T>> {
        // Note: We can load the data in a relaxed fashion because only one
        // thread should ever be mutating the vector. Since we're growing it,
        // that thread must be us.
        let old_ptr = self.ptr.load(Ordering::Relaxed);
        let old_cap = self.cap.load(Ordering::Relaxed);

        if old_cap == 0 {
            // We're growing a previously empty vector: No other readers can
            // exist so we can allocate and assign directly.
            debug_assert_eq!(self.len.load(Ordering::Relaxed), 0);
            let new_layout = Layout::array::<T>(1).unwrap();
            let new_ptr = unsafe { std::alloc::alloc(new_layout) };
            if new_ptr.is_null() {
                // If allocation fails, `new_ptr` will be null, in which case we abort.
                std::alloc::handle_alloc_error(new_layout);
            }
            // No old allocation existed to dealloc so we can return None.
            return None;
        }

        // This can't overflow because we ensure self.cap <= i32::MAX.
        let new_cap = 2 * old_cap;

        // Layout::array checks that the number of bytes is <= usize::MAX,
        // but this is redundant since old_layout.size() <= i32::MAX,
        // so the `unwrap` should never fail.
        let new_layout = Layout::array::<T>(new_cap as usize).unwrap();

        // Ensure that the new allocation doesn't exceed `i32::MAX` bytes.
        assert!(new_layout.size() <= i32::MAX as usize, "Allocation too large");

        // SAFETY: Layout guarantees that we're properly aligned for T and
        // we've made sure that new_cap is non-zero.
        let new_ptr = unsafe { std::alloc::alloc(new_layout) };

        if new_ptr.is_null() {
            // If allocation fails, `new_ptr` will be null, in which case we abort.
            std::alloc::handle_alloc_error(new_layout)
        }

        let new_ptr = new_ptr as *mut T;

        // SAFETY: src is a valid array of T with cap items.
        // dst is a valid array of T with 2 * cap items.
        // The regions cannot overlap.
        unsafe { std::ptr::copy_nonoverlapping(old_ptr, new_ptr, old_cap as usize) };
        // After this copy, the new pointer now points to a byte-for-byte copy
        // of the old pointer but with double the capacity.
        // We can store the pointer and release it to readers: Any reader can
        // acquire either the old or the new pointer.
        self.ptr.store(new_ptr, Ordering::Release);
        // Once we've stored the new pointer, we can release the
        // new capacity to users. But because the mutator thread is the only
        // thread allowed to use the new capacity, and that's us, we don't need
        // to release this.
        self.cap.store(new_cap, Ordering::Release);
        let data: Box<VecDropContainer<T>> = Box::new(VecDropContainer { ptr: NonNull::new(old_ptr).unwrap(), cap: old_cap as usize });
        Some(VecDropData(data, drop_vec_drop_container))
    }

    pub fn push(&self, elem: T) -> Option<VecDropData<T>> {
        // SAFETY: Only mutator thread is allowed to mutate; relaxed loads are fine here.
        let len = self.len.load(Ordering::Relaxed);
        let result = if len == self.cap.load(Ordering::Relaxed) {
            self.grow()
        } else {
            None
        };

        // SAFETY: self.ptr points to rw array of T; each item is properly aligned.
        unsafe {
            write(self.ptr.load(Ordering::Relaxed).add(len as usize), elem);
        }

        // Once we've written the elem to the buffer we can release the memory
        // to readers. It's tempting to use Relaxed ordering here but it's not
        // exactly correct, at least not if T is itself accessed atomically.
        self.len.store(len + 1, Ordering::Release);

        // Return the possible drop data to caller.
        result
    }

    pub fn push_mut(&mut self, elem: T) {
        let len = *self.len.get_mut();
        if len == *self.cap.get_mut() {
            self.grow_mut()
        }

        // SAFETY: self.ptr points to rw array of T; each item is properly aligned.
        unsafe {
            write(self.ptr.get_mut().add(len as usize), elem);
        }

        *self.len.get_mut() = len + 1;
    }
}