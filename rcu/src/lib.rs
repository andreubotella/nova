pub mod spmc;
pub mod vec;

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::ops::Deref;
    use std::thread;

    use crate::spmc::atomics::AtomicUsize;
    use crate::spmc::cell::{ExclusiveCell, MutatorCell};
    use crate::vec::*;

    #[test]
    fn test_rcu_vec_with_atomic_data() {
        #[derive(Debug)]
        struct ObjectData {
            child1: AtomicUsize,
            child2: AtomicUsize,
            child3: AtomicUsize,
            child4: AtomicUsize,
        }

        struct SimpleHeap {
            graveyard: ExclusiveCell<std::vec::Vec<VecDropData<ObjectData>>>,
            objects: Vec<ObjectData>,
        }

        impl Drop for SimpleHeap {
            fn drop(&mut self) {
                let _ = self.graveyard.get_mut().drain(..).for_each(|entry| {
                    drop(entry);
                });
            }
        }

        MutatorCell::new(
            SimpleHeap {
                graveyard: ExclusiveCell::new(std::vec::Vec::with_capacity(5)),
                objects: Vec::with_capacity(1024),
            },
            |cell, key| {
                let reader = cell.get_reader();
                thread::scope(|s| {
                    if let Some(drop_data) = cell.get().objects.push(
                        ObjectData {
                            child1: AtomicUsize::new(0),
                            child2: AtomicUsize::new(0),
                            child3: AtomicUsize::new(0),
                            child4: AtomicUsize::new(0),
                        },
                        &key,
                    ) {
                        cell.get().graveyard.get_exclusive_mut(&key).push(drop_data);
                    }
                    s.spawn(|| {
                        for _ in 0..10 {
                            reader.get().objects.deref().iter().for_each(|data| {
                                println!("2: {data:?}");
                            });
                        }
                    });
                    if let Some(drop_data) = cell.get().objects.push(
                        ObjectData {
                            child1: AtomicUsize::new(0),
                            child2: AtomicUsize::new(0),
                            child3: AtomicUsize::new(0),
                            child4: AtomicUsize::new(0),
                        },
                        &key,
                    ) {
                        cell.get().graveyard.get_exclusive_mut(&key).push(drop_data);
                    }
                    s.spawn(|| {
                        for _ in 0..10 {
                            reader.get().objects.deref().iter().for_each(|data| {
                                println!("1: {data:?}");
                            });
                        }
                    });
                    for _ in 0..10 {
                        if let Some(drop_data) = cell.get().objects.push(
                            ObjectData {
                                child1: AtomicUsize::new(0),
                                child2: AtomicUsize::new(0),
                                child3: AtomicUsize::new(0),
                                child4: AtomicUsize::new(0),
                            },
                            &key,
                        ) {
                            cell.get().graveyard.get_exclusive_mut(&key).push(drop_data);
                        }
                    }
                });
            },
        );
    }
}
