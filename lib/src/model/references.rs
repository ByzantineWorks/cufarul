use crate::db::Identity;

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
