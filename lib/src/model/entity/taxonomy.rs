use super::Model;
use crate::{
    db::NodeLike,
    model::{
        identity::TaxonomyId,
        property::{ReferenceProperty, TranslatableProperty},
        ReferenceKey,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Taxonomy {
    name: TranslatableProperty,
    parent: Option<ReferenceProperty>,
}

impl Model for Taxonomy {}
impl NodeLike for Taxonomy {
    type ReferenceId = ReferenceKey;

    fn references(&self) -> Vec<Self::ReferenceId> {
        if let Some(other) = &self.parent {
            let parent_id = other.value();
            return vec![ReferenceKey::OfKind(TaxonomyId::new(parent_id.id()))];
        }

        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
