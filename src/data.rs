use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Handed out by an `AssetStore` when a new asset is added.
pub struct AssetId(u32);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FileId(u32);

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

    pub fn count(&self) -> usize {
        self.assets.len()
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

trait IndexedStore {}

#[cfg(test)]
mod test {
    use super::*;

    /// When inserting new assets, the generated ids must be different.
    #[test]
    fn new_assets_should_have_non_equal_ids() {
        let mut store = AssetStore::new();
        let fid = FileId(0);

        let id_1 = store.new_asset("Asset", fid);
        let id_2 = store.new_asset("Other asset", fid);
        let id_3 = store.new_asset("Yet another asset", fid);

        assert_ne!(id_1, id_2, "Assigned ids must be unique.");
        assert_ne!(id_2, id_3, "Assigned ids must be unique.");
        assert_ne!(id_3, id_1, "Assigned ids must be unique.");
    }

    /// When adding assets, the asset count should go up.
    #[test]
    fn adding_assets_increases_count() {
        let mut store = AssetStore::new();
        let fid = FileId(0);

        store.new_asset("test", fid);
        assert_eq!(store.count(), 1);
        store.new_asset("other test", fid);
        assert_eq!(store.count(), 2);
        store.new_asset("test", fid);
        assert_eq!(store.count(), 3);
    }
}
