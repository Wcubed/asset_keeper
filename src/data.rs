use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    pub fn new_asset(&mut self, title: &str) -> AssetId {
        let id = self.next_id;
        let new_asset = Asset {
            id,
            title: title.into(),
        };

        // Store the new asset.
        self.assets.insert(id, new_asset);

        // Update where we are at with the id's.
        self.next_id = AssetId(id.0 + 1);

        return id;
    }
}

pub struct Asset {
    id: AssetId,
    title: String,
}

impl Asset {
    pub fn id(&self) -> &AssetId {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// When inserting new assets, the generated ids must be different.
    #[test]
    fn new_assets_should_have_non_equal_ids() {
        let mut store = AssetStore::new();

        let id_1 = store.new_asset("Asset");
        let id_2 = store.new_asset("Other asset");

        assert_ne!(id_1, id_2, "Assigned ids must be unique.");
    }
}
