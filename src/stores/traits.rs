use std::hash::Hash;

pub trait IndexedStore {
    type Id: StoreId;
    type Item;

    fn get(&self, id: Self::Id) -> Option<&Self::Item>;

    fn count(&self) -> usize;
}

pub trait StoreId: Eq + PartialEq + Hash + Copy + Clone {}
