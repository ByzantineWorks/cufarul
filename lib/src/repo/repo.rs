use std::fmt::Debug;

use super::{Error, RepositorySpec, Result};
use crate::{
    db::Database,
    model::{CollectionKey, ReferenceKey},
};

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
            return Err(Error::UnsupportedVersion(spec.version()));
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
}
