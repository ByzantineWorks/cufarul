use super::{Model, Query};
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
    pub name: TranslatableProperty,
    pub parent: Option<ReferenceProperty>,
}

impl Model for Taxonomy {
    fn references(&self) -> Vec<ReferenceKey> {
        if let Some(other) = &self.parent {
            let parent_id = other.value();
            return vec![ReferenceKey::ChildOf(TaxonomyId::new(parent_id.key()))];
        }

        vec![]
    }
}

impl NodeLike for Taxonomy {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Query for Taxonomy {
    fn contains(&self, value: String) -> bool {
        self.name.contains(value)
    }
}
