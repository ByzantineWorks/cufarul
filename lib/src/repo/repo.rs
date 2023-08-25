use std::fmt::Debug;

use super::{Error, RepositorySpec, Result};
use crate::db::{Database, Identity};

const SUPPORTED_VERSION: u8 = 0;

#[derive(Debug)]
pub struct Repository<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity,
{
    spec: RepositorySpec,
    db: Database<NodeId, EdgeId>,
}

impl<NodeId, EdgeId> TryFrom<RepositorySpec> for Repository<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity,
{
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

impl<NodeId, EdgeId> Repository<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity,
{
    pub fn spec(&self) -> &RepositorySpec {
        &self.spec
    }

    pub fn db_mut(&mut self) -> &mut Database<NodeId, EdgeId> {
        &mut self.db
    }

    pub fn db(&self) -> &Database<NodeId, EdgeId> {
        &self.db
    }
}
