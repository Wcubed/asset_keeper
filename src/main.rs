mod data;

fn main() {
    let mut asset_store = data::AssetStore::new();

    let new_id = asset_store.new_asset("helmet");
    println!("{:?}", new_id);
}
