use std::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    sync::Arc,
};

#[derive(Debug)]
struct NonSend<'a>(PhantomData<&'a Cell<()>>);
impl Default for NonSend<'_> {
    fn default() -> Self {
        Self(Default::default())
    }
}

/// Marker key type that opens an single-writer-multiple-reader lock for
/// reading and writing.
pub struct MutatorKey<'a>(PhantomData<NonSend<'a>>);

/// Marker key type that opens an single-writer-multiple-reader lock for
/// reading.
pub struct ReaderKey<'a>(PhantomData<&'a ()>);

/// Provides exclusive access to the contained value
///
/// The purpose of this type is to split into mutator
/// and reader cells.
#[derive(Debug)]
pub struct ExclusiveBox<T: Sync>(Box<T>);

impl<T> ExclusiveBox<T>
where
    T: Sized + Sync,
{
    pub fn new(t: T) -> Self {
        Self(Box::new(t))
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.as_mut()
    }

    // pub fn split_to_threads(self) -> MutatorCell<T> {
    //     MutatorCell::new(self.0)
    // }
}

pub struct ExclusiveCell<T: ?Sized>(UnsafeCell<T>);

impl<T: ?Sized> ExclusiveCell<T> {
    pub const fn new(t: T) -> Self
    where
        T: Sized,
    {
        Self(UnsafeCell::new(t))
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }

    pub fn get_exclusive<'a, 'b: 'a>(&'a self, _: &MutatorKey<'b>) -> &'a T {
        // SAFETY: Only mutator key can open the ExclusiveCell lock.
        unsafe { &*self.0.get() }
    }

    pub fn get_exclusive_mut<'a, 'b: 'a>(&'a self, _: &MutatorKey<'b>) -> &'a mut T {
        // SAFETY: Only mutator key can open the ExclusiveCell lock.
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl<T: ?Sized> Send for ExclusiveCell<T> {}
unsafe impl<T: ?Sized> Sync for ExclusiveCell<T> {}

#[derive(Debug)]
pub struct MutatorCell<'a, T: Sized + Sync>(Arc<T>, NonSend<'a>);

impl<'a, T: Sized + Sync> MutatorCell<'a, T> {
    pub fn new(t: T, call: impl FnOnce(MutatorCell<T>, MutatorKey<'a>)) {
        let cell = MutatorCell(Arc::new(t), Default::default());
        let key = MutatorKey(Default::default());
        call(cell, key);
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_reader(&self) -> ReaderCell<T> {
        ReaderCell::new(self.0.clone())
    }
}

// SAFETY: It is safe to change the mutator thread.
unsafe impl<'a, T: Sized + Sync> Send for MutatorCell<'a, T> {}

#[derive(Debug, Clone)]
pub struct ReaderCell<T: Sized + Sync>(Arc<T>);

impl<T: Sized + Sync> ReaderCell<T> {
    fn new(arc: Arc<T>) -> Self {
        Self(arc)
    }

    pub fn get(&self) -> &T {
        self.0.as_ref()
    }
}
