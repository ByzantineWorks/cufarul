mod composition;
mod link;
mod performance;
mod person;
mod publication;
mod taxonomy;
mod text;

use crate::model::CollectionKey;
use serde::Serialize;

pub use self::composition::{CompositionRepr, ContributionRepr};
pub use self::link::LinkRepr;
pub use self::performance::PerformanceRepr;
pub use self::person::PersonRepr;
pub use self::publication::{PublicationRepr, ReferenceInPublucationRepr};
pub use self::taxonomy::TaxonomyRepr;
pub use self::text::TextRepr;

pub type ModelReprRef = Box<dyn ModelRepr>;
pub trait ModelRepr: Send + Sync + AsBoxModelRepr + AsModelRepr + erased_serde::Serialize {}
erased_serde::serialize_trait_object!(ModelRepr);

pub trait AsModelRepr {
    fn as_model_repr(&self) -> &dyn ModelRepr;
}

pub trait AsBoxModelRepr {
    fn clone_boxed(&self) -> Box<dyn ModelRepr>;
}

impl<T: 'static + Clone + ModelRepr> AsModelRepr for T {
    fn as_model_repr(&self) -> &dyn ModelRepr {
        self
    }
}

impl<T: 'static + Clone + ModelRepr> AsBoxModelRepr for T {
    fn clone_boxed(&self) -> Box<dyn ModelRepr> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum ReferenceRepr<T>
where
    T: ModelRepr,
{
    Key(CollectionKey),
    Model(T),
    Unavailable,
}

impl<T> From<T> for ReferenceRepr<T>
where
    T: ModelRepr,
{
    fn from(value: T) -> Self {
        ReferenceRepr::Model(value)
    }
}

impl Serialize for CollectionKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
