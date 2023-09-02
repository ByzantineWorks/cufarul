use std::path::PathBuf;

pub type Index = Vec<LoadPath>;
pub struct LoadPath {
    collection: String,
    id: String,
    path: Option<PathBuf>,
}

impl From<PathBuf> for LoadPath {
    fn from(path: PathBuf) -> Self {
        /*
         * Assume that we passed a file, if not, abort since this is unexpeted.
         */
        assert!(path.is_file());

        /*
         * We confidently unwrap the first time because all files have a parent.
         */
        let collection = path
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let id = path.file_stem().unwrap().to_string_lossy().to_string();

        LoadPath {
            collection: collection,
            id: id,
            path: Some(path),
        }
    }
}

impl LoadPath {
    pub fn collection(&self) -> &String {
        &self.collection
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.path.to_owned()
    }
}
