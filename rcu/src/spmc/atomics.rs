use super::cell::MutatorKey;
use std::sync::atomic;

#[repr(transparent)]
#[derive(Debug)]
pub struct AtomicUsize(atomic::AtomicUsize);

impl AtomicUsize {
    pub const fn new(v: usize) -> Self {
        Self(atomic::AtomicUsize::new(v))
    }

    /// Returns a mutable reference to the underlying integer.
    ///
    /// This is safe because the mutable reference guarantees that no other threads are concurrently accessing the atomic data.
    pub fn get_mut(&mut self) -> &mut usize {
        self.0.get_mut()
    }

    /// Loads a value from the atomic integer.
    ///
    /// `load` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if order is [`Release`] or [`AcqRel`].
    pub fn load(&self, order: atomic::Ordering) -> usize {
        self.0.load(order)
    }

    /// Stores a value into the pointer.
    ///
    /// `store` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. Possible values are [`SeqCst`], [`Release`] and [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Acquire`] or [`AcqRel`].
    pub fn store<'a, 'b: 'a>(&'a self, val: usize, order: atomic::Ordering, _: MutatorKey<'b>) {
        self.0.store(val, order)
    }
}
