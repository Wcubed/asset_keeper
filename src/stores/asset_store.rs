use std::collections::HashMap;

use super::file_store::FileId;
use super::traits::IndexedStore;
use crate::stores::traits::StoreId;
use std::collections::hash_map::Iter;

/// Handed out by an `AssetStore` when a new asset is added.
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct AssetId(u32);

impl StoreId for AssetId {}

pub struct AssetStore {
    assets: HashMap<AssetId, Asset>,
    next_id: AssetId,
}

impl AssetStore {
    pub fn new() -> AssetStore {
        AssetStore {
            assets: HashMap::new(),
            next_id: AssetId(0),
        }
    }

    /// Creates a new asset and returns the id.
    pub fn new_asset(&mut self, title: &str, file: FileId) -> AssetId {
        let id = self.next_id;
        let new_asset = Asset {
            title: title.into(),
            file,
        };

        // Store the new asset.
        self.assets.insert(id, new_asset);

        // Update where we are at with the id's.
        self.next_id = AssetId(id.0 + 1);

        return id;
    }
}

impl IndexedStore for AssetStore {
    type Id = AssetId;
    type Item = Asset;

    fn get(&self, id: AssetId) -> Option<&Asset> {
        self.assets.get(&id)
    }

    fn count(&self) -> usize {
        self.assets.len()
    }

    fn remove(&mut self, id: &Self::Id) -> Option<Self::Item> {
        self.assets.remove(id)
    }

    fn iter(&self) -> Iter<Self::Id, Self::Item> {
        self.assets.iter()
    }
}

pub struct Asset {
    title: String,
    file: FileId,
}

impl Asset {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn file(&self) -> &FileId {
        &self.file
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stores::file_store::{FileStore, KnownExtension};
    use std::path::Path;

    /// When inserting new assets, the generated ids must be different.
    #[test]
    fn new_assets_should_have_different_ids() {
        let mut store = AssetStore::new();
        let mut file_store = FileStore::new();
        let (file_id, _) = file_store.new_file("test", KnownExtension::Png);

        let id_1 = store.new_asset("Asset", file_id);
        let id_2 = store.new_asset("Other asset", file_id);
        let id_3 = store.new_asset("Yet another asset", file_id);

        assert_ne!(id_1, id_2, "Assigned ids must be unique.");
        assert_ne!(id_2, id_3, "Assigned ids must be unique.");
        assert_ne!(id_3, id_1, "Assigned ids must be unique.");
    }

    /// When adding assets, the asset count should go up.
    #[test]
    fn adding_assets_increases_count() {
        let mut store = AssetStore::new();
        let mut file_store = FileStore::new();
        let (file_id, _) = file_store.new_file("test", KnownExtension::Png);

        store.new_asset("test", file_id);
        assert_eq!(store.count(), 1);
        store.new_asset("other test", file_id);
        assert_eq!(store.count(), 2);
        store.new_asset("test", file_id);
        assert_eq!(store.count(), 3);
    }

    #[test]
    fn getting_assets_works() {
        let mut store = AssetStore::new();
        let mut file_store = FileStore::new();
        let (file_id, _) = file_store.new_file("test", KnownExtension::Png);

        let title = "Testing";

        let new_id = store.new_asset(title, file_id);
        let asset = store.get(new_id).unwrap();

        assert_eq!(asset.title(), title);
        assert_eq!(asset.file(), &file_id);

        // Getting a non-existing asset must return None.
        assert!(store.get(AssetId(10)).is_none());
    }
}
