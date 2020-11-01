use std::collections::{HashMap, HashSet};

use super::traits::IndexedStore;
use crate::stores::traits::StoreId;
use std::collections::hash_map::Iter;
use std::path::{Path, PathBuf};

/// Handed out by a `FileStore` when a new file is added.
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct FileId(u32);

impl FileId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl StoreId for FileId {}

pub struct FileStore {
    files: HashMap<FileId, File>,
    next_id: FileId,
}

impl FileStore {
    pub fn new() -> FileStore {
        FileStore {
            files: HashMap::new(),
            next_id: FileId(0),
        }
    }

    /// Creates a new reference to a file, and returns the FileId as well as the filename that
    /// the file should be saved as.
    /// The filename is not dependant on the file's title.
    pub fn new_file(&mut self, title: &str, extension: KnownExtension) -> (FileId, PathBuf) {
        let id = self.next_id;
        let new_file = File {
            id,
            title: title.to_string(),
            extension,
            system_tags: HashSet::new(),
        };
        let file_name = new_file.file_name();

        // Store the new file.
        self.files.insert(id, new_file);

        // Update where we are at with the ids.
        self.next_id = FileId(id.0 + 1);

        (id, file_name)
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

    fn remove(&mut self, id: &Self::Id) -> Option<Self::Item> {
        self.files.remove(id)
    }

    fn iter(&self) -> Iter<Self::Id, Self::Item> {
        self.files.iter()
    }
}

pub struct File {
    id: FileId,
    title: String,
    extension: KnownExtension,
    system_tags: HashSet<SystemTag>,
}

impl File {
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn extension(&self) -> &KnownExtension {
        &self.extension
    }

    /// The file name is not dependant on the file's title.
    pub fn file_name(&self) -> PathBuf {
        PathBuf::new()
            .with_file_name(self.id.to_string())
            .with_extension(self.extension.to_str())
    }

    pub fn system_tags(&self) -> &HashSet<SystemTag> {
        &self.system_tags
    }
}
/// File extensions that we know how to deal with.
#[derive(Eq, PartialEq, Debug)]
pub enum KnownExtension {
    Png,
}

impl KnownExtension {
    /// Creates a KnownExtension from a given extension string (without the ".").
    /// Returns None when we don't know how to deal with a given type of file.
    pub fn from_str(string: &str) -> Option<KnownExtension> {
        match string.to_ascii_lowercase().as_str() {
            "png" => Some(Self::Png),
            _ => None,
        }
    }

    /// Creates a KnownExtension from a given Path.
    /// Returns None when we don't know how to deal with a given type of file.
    pub fn from_path(path: &Path) -> Option<KnownExtension> {
        Self::from_str(path.extension().unwrap_or_default().to_str().unwrap_or(""))
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Png => "png",
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum SystemTag {
    /// Indicates an image that has some kind of transparency to it.
    Transparent,
}

#[cfg(test)]
mod test_file_store {
    use super::*;

    /// When inserting new files, the generated ids must be different.
    #[test]
    fn new_files_should_have_different_ids_and_paths() {
        let mut store = FileStore::new();

        let (id_1, path_1) = store.new_file("test file", KnownExtension::Png);
        let (id_2, path_2) = store.new_file("SDKDKK@K@@", KnownExtension::Png);
        let (id_3, path_3) = store.new_file("test {}", KnownExtension::Png);

        assert_ne!(id_1, id_2, "Assigned ids must be unique.");
        assert_ne!(id_2, id_3, "Assigned ids must be unique.");
        assert_ne!(id_3, id_1, "Assigned ids must be unique.");

        assert_ne!(path_1, path_2, "Assigned paths must be unique.");
        assert_ne!(path_2, path_3, "Assigned paths must be unique.");
        assert_ne!(path_3, path_1, "Assigned paths must be unique.");
    }

    /// When adding files, the file count should go up.
    #[test]
    fn adding_files_increases_count() {
        let mut store = FileStore::new();

        store.new_file("!!!", KnownExtension::Png);
        assert_eq!(store.count(), 1);
        store.new_file("BLAA!", KnownExtension::Png);
        assert_eq!(store.count(), 2);
        store.new_file("meep!", KnownExtension::Png);
        assert_eq!(store.count(), 3);
    }

    #[test]
    fn getting_files_returns_correct_values() {
        let mut store = FileStore::new();

        let (new_id, new_name) = store.new_file("!@@#$@#@", KnownExtension::Png);
        let file = store.get(new_id).unwrap();

        // Retrieved file name must be the same as the one returned on creation.
        assert_eq!(file.file_name(), new_name);
        // The extension should match with what the KnownExtension returns as string.
        assert_eq!(
            file.file_name().extension().unwrap(),
            KnownExtension::Png.to_str()
        );

        assert_eq!(file.extension, KnownExtension::Png);

        // Getting a non-existing file must return None.
        assert!(store.get(FileId(10)).is_none());
    }
}

#[cfg(test)]
mod test_file_extensions {
    use super::*;
    use naughty_strings::BLNS;

    #[test]
    fn unknown_file_extensions_should_return_none() {
        assert!(KnownExtension::from_str("pdf").is_none());
        assert!(KnownExtension::from_str("xcf").is_none());
        assert!(KnownExtension::from_str("jpg").is_none());
    }

    #[test]
    fn file_extensions_should_work_when_capitalized() {
        assert_eq!(
            KnownExtension::from_str("PNG").unwrap(),
            KnownExtension::Png
        );
        assert_eq!(
            KnownExtension::from_str("pnG").unwrap(),
            KnownExtension::Png
        );
        assert_eq!(
            KnownExtension::from_str("PnG").unwrap(),
            KnownExtension::Png
        );
    }

    #[test]
    fn use_naughty_strings_as_extensions() {
        for string in BLNS {
            assert!(
                KnownExtension::from_str(string).is_none(),
                "This string managed to pose as a known file extension: {}",
                string
            );
        }
    }
}
