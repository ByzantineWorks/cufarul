use super::{
    core::Repository,
    error::{Error, Result},
    index::{Index, LoadPath},
    spec::RepositorySpec,
    REPOSITORY_SUPPORTED_VERSION,
};
use crate::{
    db::{Database, Datastore, EdgeId, ReferenceIdentity},
    model::{
        CollectionKey, Composition, Performance, Person, Publication, ReferenceKey, Taxonomy, Text,
    },
};
use std::sync::Arc;

type ReferenceList = Vec<EdgeId<CollectionKey, ReferenceKey>>;

pub struct CufarulRepository {
    spec: RepositorySpec,
    db: Datastore<CollectionKey, ReferenceKey>,
}

impl Repository for CufarulRepository {
    type DbType = Datastore<CollectionKey, ReferenceKey>;

    fn db(&self) -> &Self::DbType {
        &self.db
    }

    fn db_mut(&mut self) -> &mut Self::DbType {
        &mut self.db
    }

    fn index(&self) -> Result<Index> {
        let mut index = Index::new();

        for collection in CollectionKey::into_iter() {
            let base = self.spec.root().join(collection);
            let mut fragment = base
                .is_dir()
                .then_some(std::fs::read_dir(base)?)
                .ok_or(Error::MissingCollection(collection.to_owned()))
                .map(|dir| {
                    dir.filter_map(|elem| match elem {
                        Ok(entry) => Some(entry.path()),
                        Err(_) => None,
                    })
                    .filter_map(|entry| {
                        match (
                            entry.is_file(),
                            entry.extension().unwrap_or_default().to_str(),
                        ) {
                            (true, Some("toml")) => Some(LoadPath::from(entry)),
                            _ => None,
                        }
                    })
                    .collect::<Index>()
                })?;

            index.append(&mut fragment);
        }

        Ok(index)
    }

    fn sync(&mut self) -> crate::error::Result<()> {
        let mut references = ReferenceList::new();
        for entry in self.index()? {
            let path = entry.path().unwrap_or(
                self.spec
                    .root()
                    .join(entry.collection())
                    .join(entry.id())
                    .with_extension("toml"),
            );

            let key = CollectionKey::from_collection_and_id(
                entry.collection().to_owned(),
                entry.id().to_owned(),
            )?;

            println!("Loading: {key}");

            let (node, model) = match &key {
                CollectionKey::Person(_) => {
                    crate::model::into_traits(crate::model::from_file::<Person>(path)?)
                }
                CollectionKey::Composition(_) => {
                    crate::model::into_traits(crate::model::from_file::<Composition>(path)?)
                }
                CollectionKey::Performance(_) => {
                    crate::model::into_traits(crate::model::from_file::<Performance>(path)?)
                }
                CollectionKey::Publication(_) => {
                    crate::model::into_traits(crate::model::from_file::<Publication>(path)?)
                }
                CollectionKey::Text(_) => {
                    crate::model::into_traits(crate::model::from_file::<Text>(path)?)
                }
                CollectionKey::Taxonomy(_) => {
                    crate::model::into_traits(crate::model::from_file::<Taxonomy>(path)?)
                }
            };

            self.db.insert_node(key.to_owned(), node)?;
            let mut fragment: ReferenceList = model
                .references()
                .iter()
                .map(|reference| EdgeId::new(key.to_owned(), reference.to_owned()))
                .collect();
            references.append(&mut fragment);
        }

        // TODO: check if references are circular

        for edge in references {
            // Check if object exists only, for the subject we are sure since
            // the edge came from it.
            if !self.db.has_node(edge.forward_predicate().object()) {
                return Err(Error::InvalidReference(edge.to_string()).into());
            }

            self.db.insert_edge(edge, Arc::new(()))?;
        }

        Ok(())
    }
}

impl CufarulRepository {
    pub fn from_spec(spec: RepositorySpec) -> Result<Self> {
        spec.version()
            .eq(&REPOSITORY_SUPPORTED_VERSION)
            .then_some(CufarulRepository {
                spec: spec.to_owned(),
                db: Datastore::default(),
            })
            .ok_or(Error::UnsupportedVersion(spec.version()))
    }

    pub fn spec(&self) -> &RepositorySpec {
        &self.spec
    }
}
