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
    Published(super::CompositionId),
    PublishedBy(super::PublicationId),
    Performed(super::CompositionId),
    PerformedBy(super::PersonId),
    PerformerOf(super::PerformanceId),
    PerformanceBy(super::PersonId),
    ChildOf(super::TaxonomyId),
    ParentOf(super::TaxonomyId),
    OfKind(super::TaxonomyId),
    Includes(super::CompositionId),
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
            Self::PerformerOf(_) => "performer-of",
            Self::PerformanceBy(_) => "performance-by",
            Self::ChildOf(_) => "of-kind",
            Self::ParentOf(_) => "parent-of",
            Self::OfKind(_) => "of-kind",
            Self::Includes(_) => "includes",
        };

        f.write_fmt(format_args!("{} -> {}", predicate, self.object()))
    }
}

impl ReferenceIdentity<CollectionKey> for ReferenceKey {
    type Error = crate::model::Error;
    fn object(&self) -> CollectionKey {
        match self {
            Self::Authored(id) | Self::UsedIn(id) => CollectionKey::Composition(id.to_owned()),
            Self::AuthoredBy(id)
            | Self::WrittenBy(id)
            | Self::PerformedBy(id)
            | Self::PerformanceBy(id) => CollectionKey::Person(id.to_owned()),
            Self::Published(id) | Self::Includes(id) => CollectionKey::Composition(id.to_owned()),
            Self::PublishedBy(id) => CollectionKey::Publication(id.to_owned()),
            Self::Performed(id) => CollectionKey::Composition(id.to_owned()),
            Self::UsesText(id) | Self::Wrote(id) => CollectionKey::Text(id.to_owned()),
            Self::ChildOf(id) | Self::ParentOf(id) | Self::OfKind(id) => {
                CollectionKey::Taxonomy(id.to_owned())
            }
            Self::PerformerOf(id) => CollectionKey::Performance(id.to_owned()),
        }
    }

    fn reverse(&self, object: CollectionKey) -> crate::model::Result<Self> {
        Ok(match self {
            Self::Authored(_) => Self::AuthoredBy(object.try_into()?),
            Self::AuthoredBy(_) => Self::Authored(object.try_into()?),
            Self::Wrote(_) => Self::WrittenBy(object.try_into()?),
            Self::WrittenBy(_) => Self::Wrote(object.try_into()?),
            Self::UsesText(_) => Self::UsedIn(object.try_into()?),
            Self::UsedIn(_) => Self::UsesText(object.try_into()?),
            Self::Published(_) => Self::PublishedBy(object.try_into()?),
            Self::PublishedBy(_) => Self::Published(object.try_into()?),
            Self::Performed(_) => Self::PerformedBy(object.try_into()?),
            Self::PerformedBy(_) => Self::Performed(object.try_into()?),
            Self::PerformerOf(_) => Self::PerformanceBy(object.try_into()?),
            Self::PerformanceBy(_) => Self::PerformerOf(object.try_into()?),
            Self::ChildOf(_) => Self::ParentOf(object.try_into()?),
            Self::ParentOf(_) => Self::ChildOf(object.try_into()?),
            Self::OfKind(_) => Self::Includes(object.try_into()?),
            Self::Includes(_) => Self::OfKind(object.try_into()?),
        })
    }
}

impl EdgeLike for () {}
