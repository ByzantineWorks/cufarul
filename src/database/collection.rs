use crate::error::{Error, Result};

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
