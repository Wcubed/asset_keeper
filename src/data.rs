use crate::stores::asset_store::{AssetId, AssetStore};
use crate::stores::file_store::FileStore;
use std::error::Error;
use std::path::{Path, PathBuf};

pub struct Data {
    save_dir: PathBuf,
    files_dir: PathBuf,
    assets: AssetStore,
    files: FileStore,
}

impl Data {
    /// - `save_dir`: The directory to save the data files.
    /// - `files_dir`: The directory where the actual files will be stored.
    /// Will create both when they don't exist.
    pub fn new(save_dir: &Path, files_dir: &Path) -> Result<Data, Box<dyn Error>> {
        // Make sure both directories exist.
        std::fs::create_dir_all(save_dir)?;
        std::fs::create_dir_all(files_dir)?;

        Ok(Data {
            save_dir: PathBuf::from(save_dir),
            files_dir: PathBuf::from(save_dir),
            assets: AssetStore::new(),
            files: FileStore::new(),
        })
    }

    // Adds a new asset from disk. Copies the file over to the file directory.
    // Will return an error if something goes wrong during copy,
    // or if the file extension is not one we can deal with.
    /*pub fn add_asset_from_disk(
        &mut self,
        title: &str,
        file: &Path,
    ) -> Result<AssetId, Box<dyn Error>> {
        // TODO
        Ok(())
    }*/
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile;

    #[test]
    fn check_data_initialization() {
        // Setup a temporary directory for the test.
        let tempdir = tempfile::tempdir().unwrap();
        let path = tempdir.path();

        let save_dir = path.join("asset_keeper");
        let file_dir = save_dir.join("files");

        // Directories should at the moment not be there.
        assert!(!save_dir.exists());
        assert!(!file_dir.exists());

        // Initialize the data.
        let data = Data::new(&save_dir, &file_dir);

        // Now they should be there.
        assert!(save_dir.exists());
        assert!(file_dir.exists());
    }
}
