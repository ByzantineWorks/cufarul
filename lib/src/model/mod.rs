mod entity;
mod error;
mod identity;
mod property;
mod serde;

pub use self::entity::ser::{
    AsBoxModelRepr, AsModelRepr, CompositionRepr, ContributionRepr, LinkRepr, ModeRepr, ModelRepr,
    ModelReprRef, MusicalRepr, PerformanceRepr, PersonRepr, PublicationRepr,
    ReferenceInPublucationRepr, ReferenceRepr, TaxonomyRepr, TextRepr, TextVariantMap,
    TextVariantRepr, TextVariantType,
};
pub use self::entity::{
    Composition, Model, Performance, Person, Publication, Query, Taxonomy, Text,
};
pub use self::error::{Error, Result};
pub use self::identity::CollectionKey;
pub use self::identity::ReferenceKey;
pub use self::property::{Contribution, ExternalLink};
pub use self::serde::Lang;

pub use self::entity::{from_file, into_traits};
