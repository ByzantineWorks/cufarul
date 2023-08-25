use crate::db::Identity;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollectionKey {
    Person(String),
}

impl Identity for CollectionKey {}

impl From<CollectionKey> for String {
    fn from(value: CollectionKey) -> Self {
        match value {
            CollectionKey::Person(_) => "person".to_owned(),
        }
    }
}
