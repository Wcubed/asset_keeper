use crate::stores::asset_store::{Asset, AssetId, AssetStore};
use crate::stores::file_store::{FileId, FileStore, KnownExtension};
use crate::stores::traits::IndexedStore;
use anyhow::{Context, Result};
use std::collections::hash_map::Iter;
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
    pub fn new(save_dir: &Path, files_dir: &Path) -> Result<Data> {
        // Make sure both directories exist.
        std::fs::create_dir_all(save_dir).with_context(|| {
            format!(
                "Could not create save directory at: \"{}\"",
                save_dir.display()
            )
        })?;
        std::fs::create_dir_all(files_dir).with_context(|| {
            format!(
                "Could not create files directory at: \"{}\"",
                save_dir.display()
            )
        })?;

        Ok(Data {
            save_dir: PathBuf::from(save_dir),
            files_dir: PathBuf::from(save_dir),
            assets: AssetStore::new(),
            files: FileStore::new(),
        })
    }

    /// Adds a new asset from disk. Copies the file over to the file directory.
    /// Will return an error if something goes wrong during copy,
    /// or if the file extension is not one we can deal with.
    pub fn add_asset_from_disk(&mut self, title: &str, file: &Path) -> Result<AssetId> {
        let extension = KnownExtension::from_path(file).context("Extension is not known.")?;
        let (file_id, dest) = self.files.new_file(title, extension);
        let full_dest = self.files_dir.join(dest);

        match std::fs::copy(file, &full_dest) {
            Ok(_) => {}
            Err(e) => {
                // The file is not actually in the save folder.
                // Make sure we don't leave an orphaned reference in the storage.
                self.files.remove(&file_id);
                return Err(e).with_context(|| {
                    format!(
                        "Could not copy asset \"{}\" to the file storage at \"{}\"",
                        file.display(),
                        full_dest.display()
                    )
                });
            }
        }

        let asset_id = self.assets.new_asset(title, file_id);

        Ok(asset_id)
    }

    pub fn asset_count(&self) -> usize {
        self.assets.count()
    }

    pub fn assets_iter(&self) -> Iter<AssetId, Asset> {
        self.assets.iter()
    }

    pub fn get_asset(&self, id: AssetId) -> Option<&Asset> {
        self.assets.get(id)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile;
    use tempfile::TempDir;

    const TEST_FILES_PATH: &str = "tests/files";

    #[test]
    fn check_data_initialization() {
        // Setup a temporary directory for the test.
        let (_, save_dir, file_dir) = setup_temp_directory();

        // Directories should at the moment not be there.
        assert!(!save_dir.exists());
        assert!(!file_dir.exists());

        // Initialize the data.
        let data = Data::new(&save_dir, &file_dir);

        // Now they should be there.
        assert!(save_dir.exists());
        assert!(file_dir.exists());
    }

    #[test]
    fn add_assets() -> Result<()> {
        // Setup a temporary directory for the test.
        let (_, save_dir, file_dir) = setup_temp_directory();
        // Initialize the data.
        let mut data = Data::new(&save_dir, &file_dir).unwrap();

        let test_files = Path::new(TEST_FILES_PATH);

        let title = "Testing title";

        let id = data.add_asset_from_disk(title, &test_files.join(Path::new("swords/tall.png")))?;

        // Check if it has been created properly.
        assert_eq!(data.asset_count(), 1);
        let asset = data.get_asset(id).unwrap();
        assert_eq!(asset.title(), title);

        // TODO: Check if the file can be retrieved as well.

        Ok(())
    }

    // TODO: add a check for adding nonexisting asset files
    //       and ones with an extension we dont recognise.

    /// Sets up a temporary directory for use in the other tests
    /// The directory will disappear as soon as the directory handle goes out of scope.
    /// Returns:
    /// - (Temporary directory handle,
    /// - save directory path (does not exist yet),
    /// - files directory path (also does not exist yet))
    fn setup_temp_directory() -> (TempDir, PathBuf, PathBuf) {
        let tempdir = tempfile::tempdir().unwrap();
        let path = tempdir.path();

        let save_dir = path.join("asset_keeper");
        let file_dir = save_dir.join("files");

        (tempdir, save_dir, file_dir)
    }
}
