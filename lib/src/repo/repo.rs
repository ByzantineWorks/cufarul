use super::{LoadSpec, RepositorySpec};
use crate::{
    db::{Database, INode, ReferenceList},
    error::{Error, Result},
    model::{CollectionKey, Person, ReferenceKey},
};
use std::{collections::HashMap, fmt::Debug, rc::Rc};

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
        /* Todo: can the types be defined in one place? */
        let mut reference_map: HashMap<CollectionKey, ReferenceList<CollectionKey, ReferenceKey>> =
            HashMap::new();

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
            let data: Rc<dyn INode<NodeId = CollectionKey, EdgeId = ReferenceKey>> = match &key {
                CollectionKey::Person(_) => crate::model::from_file::<Person>(path)?,
            };

            let _ = self.db.insert_node(key.to_owned(), data.to_owned());
            reference_map.insert(key, data.references());
        }

        /* Todo: check if references are circular */

        /* parse references */
        for (from, spec) in reference_map {
            for dest in spec {
                /* check if target exists */
                if !self.db.has_node(dest.target.to_owned()) {
                    return Err(super::Error::InvalidReference(dest.target.to_string()).into());
                }

                let _ = self
                    .db
                    .insert_edge(from.to_owned(), dest.target, dest.predicate, None)?;
            }
        }

        Ok(())
    }
}
