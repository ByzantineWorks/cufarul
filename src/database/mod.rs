mod collection;
mod loader;
mod spec;
mod version;

pub use self::collection::CollectionKey;

use self::{loader::LoadSpec, spec::DatabaseSpec, version::DATABASE_VERSION};
use crate::{
    error::{Error, Result},
    models::{from_file, Model, Person},
    serde::NonEmptyString,
};
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
    rc::Rc,
};

type GenericCollection = BTreeMap<NonEmptyString, Rc<dyn Model>>;
type CollectionMap = HashMap<CollectionKey, GenericCollection>;

/*
 * Database - internal representation of the database
 */
pub struct Database {
    spec: DatabaseSpec,
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

        let db_spec = DatabaseSpec::try_from(current.join(".cufarul"))?;
        let load_spec = LoadSpec::try_from(db_spec.clone())?;
        let mut db = Database::try_from(db_spec)?;

        for entry in load_spec {
            let collection = db.collections.get_mut(&entry.collection).unwrap();
            match entry.collection {
                CollectionKey::People => {
                    collection.insert(
                        NonEmptyString::try_from(entry.id)?,
                        Rc::new(from_file::<Person>(entry.path)?),
                    );
                }
            }
        }

        Ok(db)
    }

    pub fn root(&self) -> &PathBuf {
        &self.spec.root
    }

    pub fn collection(&self, key: CollectionKey) -> &GenericCollection {
        &self.collections.get(&key).unwrap()
    }
}

impl TryFrom<DatabaseSpec> for Database {
    type Error = Error;
    fn try_from(spec: DatabaseSpec) -> Result<Self> {
        if spec.database.version.major() != DATABASE_VERSION {
            return Err(Error::UnsupportedDatabaseVersion(
                spec.database.version.major(),
            ));
        }

        let mut collections = CollectionMap::new();
        for c in CollectionKey::iter() {
            collections.insert(c.clone(), GenericCollection::new());
        }

        Ok(Database {
            spec: spec.to_owned(),
            collections: collections,
        })
    }
}
