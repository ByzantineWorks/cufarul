use super::{Model, Query};
use crate::{
    db::NodeLike,
    model::{
        identity::PersonId,
        property::{LinkProperty, ReferenceProperty},
        ReferenceKey,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Performance {
    pub performer: ReferenceProperty,
    pub link: LinkProperty,
}

impl Model for Performance {
    fn references(&self) -> Vec<ReferenceKey> {
        let author_id = self.performer.value();
        vec![ReferenceKey::PerformedBy(PersonId::new(author_id.key()))]
    }
}

impl NodeLike for Performance {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Query for Performance {
    fn contains(&self, value: String) -> bool {
        false
    }
}
