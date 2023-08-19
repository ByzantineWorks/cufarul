use crate::error::{Error, Result};
use std::slice::Iter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CollectionKey {
    People,
}

impl TryFrom<String> for CollectionKey {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        match value.trim() {
            "people" => Ok(CollectionKey::People),
            other => Err(Error::InvalidCollection(other.to_owned())),
        }
    }
}

impl From<CollectionKey> for String {
    fn from(value: CollectionKey) -> Self {
        match value {
            CollectionKey::People => String::from("people"),
        }
    }
}

impl CollectionKey {
    pub fn iter() -> Iter<'static, CollectionKey> {
        const COLLECTIONS: [CollectionKey; 1] = [CollectionKey::People];
        COLLECTIONS.iter()
    }
}
