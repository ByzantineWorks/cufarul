use crate::{
    db::Identity,
    repo::{Error, Result},
};
use std::{fmt::Display, slice::Iter};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectionKey {
    Person(String),
}

impl Identity for CollectionKey {}

impl From<CollectionKey> for String {
    fn from(value: CollectionKey) -> Self {
        match value {
            CollectionKey::Person(_) => "people".to_owned(),
        }
    }
}

impl Display for CollectionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Person(id) => {
                f.write_fmt(format_args!("@{}/{}", String::from(self.to_owned()), id))
            }
        }
    }
}

impl CollectionKey {
    pub fn iter() -> Iter<'static, &'static str> {
        const COLLECTIONS: [&str; 1] = ["people"];
        COLLECTIONS.iter()
    }

    pub fn new(key: &str, id: String) -> Result<Self> {
        match key {
            "people" | "person" => Ok(CollectionKey::Person(id)),
            _ => Err(Error::InvalidCollectionKey(key.to_owned())),
        }
    }
}
