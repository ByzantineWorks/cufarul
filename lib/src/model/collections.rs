use crate::{
    db::NodeIdentity,
    repo::{Error, Result},
};
use std::fmt::Display;

use super::{CompositionId, PersonId};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectionKey {
    Person(PersonId),
    Composition(CompositionId),
}

impl Display for CollectionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (collection, id) = match self {
            Self::Person(id) => ("people", id.to_string()),
            Self::Composition(id) => ("compositions", id.to_string()),
        };

        f.write_fmt(format_args!("@{}/{}", collection, id))
    }
}

impl CollectionKey {
    pub fn into_iter() -> std::array::IntoIter<&'static str, 2> {
        const COLLECTIONS: [&str; 2] = ["people", "compositions"];
        COLLECTIONS.into_iter()
    }

    pub fn from_collection_and_id(collection: String, id: String) -> Result<Self> {
        match collection.as_str() {
            "people" => Ok(CollectionKey::Person(PersonId::new(id))),
            "compositions" => Ok(CollectionKey::Composition(CompositionId::new(id))),
            _ => Err(Error::InvalidCollectionKey(collection)),
        }
    }
}

impl NodeIdentity for CollectionKey {}
