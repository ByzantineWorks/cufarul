use super::{LoadSpec, RepositorySpec};
use crate::{
    db::Database,
    error::{Error, Result},
    model::{CollectionKey, Model, Person, ReferenceKey},
};
use std::fmt::Debug;

const SUPPORTED_VERSION: u8 = 0;

#[derive(Debug)]
pub struct Repository {
    spec: RepositorySpec,
    db: Database<CollectionKey, ReferenceKey>,
}

impl TryFrom<RepositorySpec> for Repository {
    type Error = Error;
    fn try_from(spec: RepositorySpec) -> Result<Self> {
        if spec.version() != SUPPORTED_VERSION {
            return Err(crate::repo::Error::UnsupportedVersion(spec.version()).into());
        }

        Ok(Repository {
            spec: spec,
            db: Database::default(),
        })
    }
}

impl Repository {
    pub fn spec(&self) -> &RepositorySpec {
        &self.spec
    }

    pub fn db_mut(&mut self) -> &mut Database<CollectionKey, ReferenceKey> {
        &mut self.db
    }

    pub fn db(&self) -> &Database<CollectionKey, ReferenceKey> {
        &self.db
    }

    pub fn sync(&mut self) -> Result<()> {
        let load_spec = LoadSpec::try_from(self.spec.to_owned())?;
        for entry in load_spec {
            let path = entry.path().unwrap_or(
                self.spec
                    .root()
                    .join(entry.collection())
                    .join(entry.id())
                    .with_extension("toml"),
            );

            let key = CollectionKey::new(&entry.collection(), entry.id().to_owned())?;
            let data = match key {
                CollectionKey::Person(_) => Person::load(path)?,
            };

            let _ = self.db.insert_node(key, data);
        }

        Ok(())
    }
}
