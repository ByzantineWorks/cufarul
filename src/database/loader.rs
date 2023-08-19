use super::{spec::DatabaseSpec, CollectionKey};
use crate::error::{Error, Result};
use std::path::PathBuf;

/*
 * LoadPath - a single entry that needs to be loaded from disk into a model
 */
#[derive(Debug)]
pub struct LoadPath {
    pub collection: CollectionKey,
    pub id: String,
    pub path: PathBuf,
}

impl TryFrom<PathBuf> for LoadPath {
    type Error = Error;
    fn try_from(path: PathBuf) -> Result<Self> {
        /*
         * assume that we passed a file, if not, abort since this is not a
         * user-level error.
         */
        assert!(path.is_file());

        let collection_str = path
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let id_str = path.file_stem().unwrap().to_string_lossy().to_string();

        Ok(LoadPath {
            collection: CollectionKey::try_from(collection_str)?,
            id: id_str,
            path: path,
        })
    }
}

impl From<LoadPath> for PathBuf {
    fn from(value: LoadPath) -> Self {
        value.path
    }
}

pub type LoadSpec = Vec<LoadPath>;

impl TryFrom<DatabaseSpec> for LoadSpec {
    type Error = Error;
    fn try_from(spec: DatabaseSpec) -> Result<Self> {
        let mut res = LoadSpec::new();
        for collection in CollectionKey::iter() {
            let collection_dir = spec.root.join(String::from(collection.to_owned()));
            for entry in std::fs::read_dir(collection_dir)? {
                let path = entry?.path();
                if path.is_file() && path.extension().unwrap_or_default() == "toml" {
                    res.push(LoadPath::try_from(path)?);
                }
            }
        }

        Ok(res)
    }
}
