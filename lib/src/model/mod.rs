mod person;

pub use person::Person;

use crate::db::Identity;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectionKey {
    Person(String),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReferenceKey {
    Author,
}

impl Identity for CollectionKey {}
impl Identity for ReferenceKey {}

impl From<CollectionKey> for String {
    fn from(value: CollectionKey) -> Self {
        match value {
            CollectionKey::Person(_) => "person".to_owned(),
        }
    }
}

impl From<ReferenceKey> for String {
    fn from(value: ReferenceKey) -> Self {
        match value {
            ReferenceKey::Author => "author".to_owned(),
        }
    }
}
