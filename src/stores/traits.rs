use std::collections::hash_map::Iter;
use std::hash::Hash;

pub trait IndexedStore {
    type Id: StoreId;
    type Item;

    fn get(&self, id: Self::Id) -> Option<&Self::Item>;

    fn count(&self) -> usize;

    fn remove(&mut self, id: &Self::Id) -> Option<Self::Item>;

    fn iter(&self) -> Iter<Self::Id, Self::Item>;
}

pub trait StoreId: Eq + PartialEq + Hash + Copy + Clone {}
