use super::CollectionKey;
use crate::db::{EdgeLike, ReferenceIdentity};
use std::fmt::Display;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReferenceKey {
    Authored(super::CompositionId),
    AuthoredBy(super::PersonId),
    Wrote(super::TextId),
    WrittenBy(super::PersonId),
    UsesText(super::TextId),
    UsedIn(super::CompositionId),
    Published(super::PublicationId),
    PublishedBy(super::PersonId),
    Performed(super::PerformanceId),
    PerformedBy(super::PersonId),
    OfKind(super::TaxonomyId),
    ParentOf(super::TaxonomyId),
}

impl Display for ReferenceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let predicate = match self {
            Self::Authored(_) => "authored",
            Self::AuthoredBy(_) => "authored-by",
            Self::Wrote(_) => "wrote",
            Self::WrittenBy(_) => "written-by",
            Self::UsesText(_) => "uses-text",
            Self::UsedIn(_) => "used-in",
            Self::Published(_) => "published",
            Self::PublishedBy(_) => "published-by",
            Self::Performed(_) => "performed",
            Self::PerformedBy(_) => "performed-by",
            Self::OfKind(_) => "of-kind",
            Self::ParentOf(_) => "parent-of",
        };

        f.write_fmt(format_args!("{} -> {}", predicate, self.object()))
    }
}

impl ReferenceIdentity<CollectionKey> for ReferenceKey {
    fn object(&self) -> CollectionKey {
        match self {
            Self::Authored(id) | Self::UsedIn(id) => CollectionKey::Composition(id.to_owned()),
            Self::AuthoredBy(id)
            | Self::WrittenBy(id)
            | Self::PublishedBy(id)
            | Self::PerformedBy(id) => CollectionKey::Person(id.to_owned()),
            Self::Published(id) => CollectionKey::Publication(id.to_owned()),
            Self::Performed(id) => CollectionKey::Performance(id.to_owned()),
            Self::UsesText(id) | Self::Wrote(id) => CollectionKey::Text(id.to_owned()),
            Self::OfKind(id) | Self::ParentOf(id) => CollectionKey::Taxonomy(id.to_owned()),
        }
    }
}

impl EdgeLike for () {}
