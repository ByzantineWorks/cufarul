use std::fmt::Display;

use crate::db::{Allowed, Identity};

use super::CollectionKey;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReferenceKey {
    Author,
}

impl Identity for ReferenceKey {}

impl From<ReferenceKey> for String {
    fn from(value: ReferenceKey) -> Self {
        match value {
            ReferenceKey::Author => "author".to_owned(),
        }
    }
}

impl Allowed<CollectionKey> for ReferenceKey {
    fn is_allowed(&self, from: &CollectionKey, to: &CollectionKey) -> bool {
        match (from, self, to) {
            _ => false,
        }
    }
}

impl Display for ReferenceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from(self.to_owned()).as_str())
    }
}
