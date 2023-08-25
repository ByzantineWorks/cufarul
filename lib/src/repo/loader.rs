use super::{Error, RepositorySpec, Result};
use crate::model::CollectionKey;
use std::path::PathBuf;

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

pub type LoadSpec = Vec<LoadPath>;

impl TryFrom<RepositorySpec> for LoadSpec {
    type Error = Error;
    fn try_from(spec: RepositorySpec) -> Result<Self> {
        let mut res = LoadSpec::new();
        for collection in CollectionKey::iter() {
            let collection_dir = spec.root().join(collection.to_owned());
            for entry in std::fs::read_dir(collection_dir)? {
                let path = entry?.path();
                if path.is_file() && path.extension().unwrap_or_default() == "toml" {
                    res.push(LoadPath::from(path));
                }
            }
        }

        Ok(res)
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
