use super::{
    core::Repository,
    error::{Error, Result},
    index::{Index, LoadPath},
    spec::RepositorySpec,
    REPOSITORY_SUPPORTED_VERSION,
};
use crate::{
    db::{Database, Datastore, EdgeId, NodeLike, ReferenceIdentity},
    model::{CollectionKey, Composition, Person, ReferenceKey},
};
use std::rc::Rc;

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
                        Ok(entry) => Some(LoadPath::from(entry.path())),
                        Err(_) => None,
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
            let data: Rc<dyn NodeLike<ReferenceId = ReferenceKey>> = match &key {
                CollectionKey::Person(_) => crate::model::from_file::<Person>(path)?,
                CollectionKey::Composition(_) => crate::model::from_file::<Composition>(path)?,
            };

            self.db.insert_node(key.to_owned(), data.clone())?;
            let mut fragment: ReferenceList = data
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
            if !self.db.has_node(edge.predicate().object()) {
                return Err(Error::InvalidReference(edge.to_string()).into());
            }

            self.db.insert_edge(edge, Rc::new(()))?;
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
