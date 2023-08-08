use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use crate::{
    error::{Error, Result},
    models::{Model, Person},
    serde::NonEmptyString,
};

type GenericCollection = BTreeMap<NonEmptyString, Box<dyn Model>>;
type CollectionMap = HashMap<CollectionKey, GenericCollection>;

#[derive(Eq, PartialEq, Hash)]
pub enum CollectionKey {
    People,
}

pub struct Database {
    root: PathBuf,
    collections: CollectionMap,
}

impl Database {
    pub fn load(root: &Path) -> Result<Self> {
        let mut current = std::fs::canonicalize(root)?;

        loop {
            if current.join(".cufarul").exists() {
                break;
            }

            current = match current.parent() {
                Some(p) => p.canonicalize()?,
                None => return Err(Error::NoDatabase),
            };
        }

        let mut c: CollectionMap = CollectionMap::new();
        c.insert(CollectionKey::People, GenericCollection::new());

        let people_entries = std::fs::read_dir(current.join("people"))?;
        for entry in people_entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == "toml" {
                    let tmp = path.clone();
                    let p: Person = crate::models::from_file(path)?;
                    let stem = tmp.file_stem().unwrap().to_string_lossy().to_string();
                    let id: NonEmptyString = NonEmptyString::try_from(stem)?;
                    c.get_mut(&CollectionKey::People)
                        .unwrap()
                        .insert(id, Box::new(p));
                }
            }
        }

        Ok(Database {
            root: current.to_owned(),
            collections: c,
        })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn collection(&self, key: CollectionKey) -> &GenericCollection {
        &self.collections.get(&key).unwrap()
    }
}
