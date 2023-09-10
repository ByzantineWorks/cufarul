use super::CollectionKey;
use crate::db::{EdgeLike, ReferenceIdentity};
use std::fmt::Display;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReferenceKey {
    Authored(super::Composition),
    AuthoredBy(super::Person),
}

impl Display for ReferenceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let predicate = match self {
            Self::Authored(_) => "authored",
            Self::AuthoredBy(_) => "authored-by",
        };

        f.write_fmt(format_args!("{} -> {}", predicate, self.object()))
    }
}

impl ReferenceIdentity<CollectionKey> for ReferenceKey {
    fn object(&self) -> CollectionKey {
        match self {
            ReferenceKey::Authored(id) => CollectionKey::Composition(id.to_owned()),
            ReferenceKey::AuthoredBy(id) => CollectionKey::Person(id.to_owned()),
        }
    }
}

impl EdgeLike for () {}
