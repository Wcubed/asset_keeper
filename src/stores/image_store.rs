use std::collections::HashMap;

use super::traits::IndexedStore;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Handed out by a `ImageStore` when a new asset is added.
pub struct ImageId(u32);

pub struct ImageStore {
    files: HashMap<ImageId, Image>,
    next_id: ImageId,
}

/// Extensions that we recognize as images.
pub const IMAGE_EXTENSIONS: [&str; 1] = ["png"];

impl ImageStore {
    pub fn new() -> ImageStore {
        ImageStore {
            files: HashMap::new(),
            next_id: ImageId(0),
        }
    }

    /// Creates a new reference to an image, and returns the id.
    /// Will return `None` if the path's extension is not in `IMAGE_EXTENSIONS`.
    pub fn new_image(&mut self, path: &Path) -> Option<ImageId> {
        if !ImageStore::path_has_image_extension(path) {
            // This is not an image path we recognize.
            return None;
        }

        let id = self.next_id;
        let new_image = Image {
            path: PathBuf::from(path),
        };

        // Store the new image.
        self.files.insert(id, new_image);

        // Update where we are at with the ids.
        self.next_id = ImageId(id.0 + 1);

        Some(id)
    }

    /// Does the path have an extension that we recognize as being an image?
    /// This is true if the extension is in `IMAGE_EXTENSIONS`.
    /// Does not care about capitalization.
    pub fn path_has_image_extension(path: &Path) -> bool {
        match path.extension() {
            Some(ext) => {
                if let Some(string) = ext.to_str() {
                    let lowercase = string.to_lowercase();
                    IMAGE_EXTENSIONS.contains(&lowercase.as_str())
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

impl IndexedStore for ImageStore {
    type Id = ImageId;
    type Item = Image;

    fn get(&self, id: ImageId) -> Option<&Image> {
        self.files.get(&id)
    }

    fn count(&self) -> usize {
        self.files.len()
    }
}

pub struct Image {
    path: PathBuf,
}

impl Image {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use naughty_strings::BLNS;

    /// When inserting new images, the generated ids must be different.
    #[test]
    fn new_images_should_have_different_ids() {
        let mut store = ImageStore::new();

        let id_1 = store.new_image(Path::new("image.png")).unwrap();
        let id_2 = store.new_image(Path::new("other.png")).unwrap();
        let id_3 = store.new_image(Path::new("test.png")).unwrap();

        assert_ne!(id_1, id_2, "Assigned ids must be unique.");
        assert_ne!(id_2, id_3, "Assigned ids must be unique.");
        assert_ne!(id_3, id_1, "Assigned ids must be unique.");
    }

    /// When adding images, the image count should go up.
    #[test]
    fn adding_images_increases_count() {
        let mut store = ImageStore::new();
        let path = Path::new("test.png");

        store.new_image(path);
        assert_eq!(store.count(), 1);
        store.new_image(path);
        assert_eq!(store.count(), 2);
        store.new_image(path);
        assert_eq!(store.count(), 3);
    }

    #[test]
    fn getting_images_works() {
        let mut store = ImageStore::new();

        let path = Path::new("images/test.png");

        let new_id = store.new_image(path).unwrap();
        let image = store.get(new_id).unwrap();

        assert_eq!(image.path().as_os_str(), path.as_os_str());

        // Getting a non-existing image must return None.
        assert!(store.get(ImageId(10)).is_none());
    }

    #[test]
    fn non_image_extensions_should_be_rejected() {
        let mut store = ImageStore::new();

        assert!(store.new_image(Path::new("test.pdf")).is_none());
        assert!(store.new_image(Path::new("blaargh!")).is_none());
        assert!(store.new_image(Path::new("image/test/bla.jpg")).is_none());
    }

    #[test]
    fn image_extensions_should_work_when_capitalized() {
        let mut store = ImageStore::new();

        assert!(store.new_image(Path::new("image.PNG")).is_some());
    }

    #[test]
    fn use_naughty_strings_as_paths() {
        let mut store = ImageStore::new();

        for string in BLNS {
            let path = Path::new(string);
            assert!(
                store.new_image(path).is_none(),
                "This string managed to pose as an image path: {}",
                string
            );
        }
    }
}
