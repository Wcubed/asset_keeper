pub trait IndexedStore {
    type Id;
    type Item;

    fn get(&self, id: Self::Id) -> Option<&Self::Item>;

    fn count(&self) -> usize;
}
