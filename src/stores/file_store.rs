use std::collections::HashMap;

use super::traits::IndexedStore;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Handed out by a `FileStore` when a new file is added.
pub struct FileId(u32);

pub struct FileStore {
    files: HashMap<FileId, File>,
    next_id: FileId,
}

/// Extensions that we know how to deal with.
pub const KNOWN_EXTENSIONS: [&str; 1] = ["png"];

impl FileStore {
    pub fn new() -> FileStore {
        FileStore {
            files: HashMap::new(),
            next_id: FileId(0),
        }
    }

    /// Creates a new reference to an file, and returns the id.
    /// TODO: Will copy the given file to the applications file storage location.
    /// Will return `None` if the path's extension is not in [`KNOWN_EXTENSIONS`](KNOWN_EXTENSIONS).
    /// TODO: or if the file copy failed.
    pub fn new_file_from_disk(&mut self, path: &Path) -> Option<FileId> {
        if !FileStore::path_has_file_extension(path) {
            // This is not an file path we recognize.
            return None;
        }

        let id = self.next_id;
        let new_file = File {
            path: PathBuf::from(path),
        };

        // Store the new file.
        self.files.insert(id, new_file);

        // Update where we are at with the ids.
        self.next_id = FileId(id.0 + 1);

        Some(id)
    }

    /// Does the path have an extension that we recognize as being an file?
    /// This is true if the extension is in [`KNOWN_EXTENSIONS`](KNOWN_EXTENSIONS).
    /// Does not care about capitalization.
    pub fn path_has_file_extension(path: &Path) -> bool {
        match path.extension() {
            Some(ext) => {
                if let Some(string) = ext.to_str() {
                    let lowercase = string.to_lowercase();
                    KNOWN_EXTENSIONS.contains(&lowercase.as_str())
                } else {
                    false
                }
            }
            None => {
                // Path does not have an extension.
                false
            }
        }
    }
}

impl IndexedStore for FileStore {
    type Id = FileId;
    type Item = File;

    fn get(&self, id: FileId) -> Option<&File> {
        self.files.get(&id)
    }

    fn count(&self) -> usize {
        self.files.len()
    }
}

pub struct File {
    path: PathBuf,
}

impl File {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use naughty_strings::BLNS;

    /// When inserting new files, the generated ids must be different.
    #[test]
    fn new_files_should_have_different_ids() {
        let mut store = FileStore::new();

        let id_1 = store.new_file_from_disk(Path::new("file.png")).unwrap();
        let id_2 = store.new_file_from_disk(Path::new("other.png")).unwrap();
        let id_3 = store.new_file_from_disk(Path::new("test.png")).unwrap();

        assert_ne!(id_1, id_2, "Assigned ids must be unique.");
        assert_ne!(id_2, id_3, "Assigned ids must be unique.");
        assert_ne!(id_3, id_1, "Assigned ids must be unique.");
    }

    /// When adding files, the file count should go up.
    #[test]
    fn adding_files_increases_count() {
        let mut store = FileStore::new();
        let path = Path::new("test.png");

        store.new_file_from_disk(path);
        assert_eq!(store.count(), 1);
        store.new_file_from_disk(path);
        assert_eq!(store.count(), 2);
        store.new_file_from_disk(path);
        assert_eq!(store.count(), 3);
    }

    #[test]
    fn getting_files_works() {
        let mut store = FileStore::new();

        let path = Path::new("files/test.png");

        let new_id = store.new_file_from_disk(path).unwrap();
        let file = store.get(new_id).unwrap();

        assert_eq!(file.path().as_os_str(), path.as_os_str());

        // Getting a non-existing file must return None.
        assert!(store.get(FileId(10)).is_none());
    }

    #[test]
    fn unknown_file_extensions_should_be_rejected() {
        let mut store = FileStore::new();

        assert!(store.new_file_from_disk(Path::new("test.pdf")).is_none());
        assert!(store.new_file_from_disk(Path::new("blaargh!")).is_none());
        assert!(store
            .new_file_from_disk(Path::new("file/test/bla.jpg"))
            .is_none());
    }

    #[test]
    fn file_extensions_should_work_when_capitalized() {
        let mut store = FileStore::new();

        assert!(store.new_file_from_disk(Path::new("file.PNG")).is_some());
    }

    #[test]
    fn use_naughty_strings_as_paths() {
        let mut store = FileStore::new();

        for string in BLNS {
            let path = Path::new(string);
            assert!(
                store.new_file_from_disk(path).is_none(),
                "This string managed to pose as a known file path: {}",
                string
            );
        }
    }
}
