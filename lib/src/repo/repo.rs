use std::{fmt::Debug, hash::Hash};

use super::{Error, RepositorySpec, Result};
use crate::db::{Database, Identify};

const SUPPORTED_VERSION: u8 = 0;

#[derive(Debug)]
pub struct Repository<E, N>
where
    E: Debug + Hash + Eq + Identify,
    N: Debug,
{
    spec: RepositorySpec,
    db: Database<E, N>,
}

impl<E, N> TryFrom<RepositorySpec> for Repository<E, N>
where
    E: Debug + Hash + Eq + Identify,
    N: Debug,
{
    type Error = Error;
    fn try_from(spec: RepositorySpec) -> Result<Self> {
        if spec.version() != SUPPORTED_VERSION {
            return Err(Error::UnsupportedVersion(spec.version()));
        }

        Ok(Repository {
            spec: spec,
            db: Database::<E, N>::default(),
        })
    }
}

impl<E, N> Repository<E, N>
where
    E: Debug + Hash + Eq + Identify,
    N: Debug,
{
    pub fn spec(&self) -> &RepositorySpec {
        &self.spec
    }

    pub fn db_mut(&mut self) -> &mut Database<E, N> {
        &mut self.db
    }

    pub fn db(&self) -> &Database<E, N> {
        &self.db
    }
}
