use std::{hash::Hash, collections::HashSet};

pub trait InsertAll<T>
where
    T: IntoIterator,
    T::Item: Eq,
    T::Item: Hash,
    T::Item: Copy,
{
    fn insert_all(&mut self, collection: T);
}

impl<T> InsertAll<T> for HashSet<T::Item>
where
    T: IntoIterator,
    T::Item: Eq,
    T::Item: Hash,
    T::Item: Copy,
{
    fn insert_all(&mut self, collection: T) {
        for item in collection.into_iter() {
            self.insert(item.clone());
        }
    }
}
