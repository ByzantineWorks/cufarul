mod entity;
mod error;
mod identity;
mod property;
mod serde;

pub use self::entity::ser::{
    AsBoxModelRepr, AsModelRepr, CompositionRepr, ContributionRepr, LinkRepr, ModelRepr,
    ModelReprRef, PerformanceRepr, PersonRepr, PublicationRepr, ReferenceInPublucationRepr,
    ReferenceRepr, TaxonomyRepr, TextRepr,
};
pub use self::entity::{Composition, Model, Performance, Person, Publication, Taxonomy, Text};
pub use self::error::{Error, Result};
pub use self::identity::CollectionKey;
pub use self::identity::ReferenceKey;

pub use self::entity::{from_file, into_traits};
