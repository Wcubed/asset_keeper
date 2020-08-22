use std::collections::HashMap;

use super::image_store::ImageId;
use super::traits::IndexedStore;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Handed out by an `AssetStore` when a new asset is added.
pub struct AssetId(u32);

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
    pub fn new_asset(&mut self, title: &str, image: ImageId) -> AssetId {
        let id = self.next_id;
        let new_asset = Asset {
            title: title.into(),
            image,
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
}

pub struct Asset {
    title: String,
    image: ImageId,
}

impl Asset {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn image(&self) -> &ImageId {
        &self.image
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stores::image_store::ImageStore;
    use std::path::Path;

    /// When inserting new assets, the generated ids must be different.
    #[test]
    fn new_assets_should_have_different_ids() {
        let mut store = AssetStore::new();
        let mut image_store = ImageStore::new();
        let img_id = image_store.new_image(Path::new("Test.png")).unwrap();

        let id_1 = store.new_asset("Asset", img_id);
        let id_2 = store.new_asset("Other asset", img_id);
        let id_3 = store.new_asset("Yet another asset", img_id);

        assert_ne!(id_1, id_2, "Assigned ids must be unique.");
        assert_ne!(id_2, id_3, "Assigned ids must be unique.");
        assert_ne!(id_3, id_1, "Assigned ids must be unique.");
    }

    /// When adding assets, the asset count should go up.
    #[test]
    fn adding_assets_increases_count() {
        let mut store = AssetStore::new();
        let mut image_store = ImageStore::new();
        let img_id = image_store.new_image(Path::new("Test.png")).unwrap();

        store.new_asset("test", img_id);
        assert_eq!(store.count(), 1);
        store.new_asset("other test", img_id);
        assert_eq!(store.count(), 2);
        store.new_asset("test", img_id);
        assert_eq!(store.count(), 3);
    }

    #[test]
    fn getting_assets_works() {
        let mut store = AssetStore::new();
        let mut image_store = ImageStore::new();
        let img_id = image_store.new_image(Path::new("Test.png")).unwrap();

        let title = "Testing";

        let new_id = store.new_asset(title, img_id);
        let asset = store.get(new_id).unwrap();

        assert_eq!(asset.title(), title);
        assert_eq!(asset.image(), &img_id);

        // Getting a non-existing asset must return None.
        assert!(store.get(AssetId(10)).is_none());
    }
}
