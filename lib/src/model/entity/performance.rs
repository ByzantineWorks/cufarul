use super::Model;
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
    performer: ReferenceProperty,
    link: LinkProperty,
}

impl Model for Performance {}
impl NodeLike for Performance {
    type ReferenceId = ReferenceKey;

    fn references(&self) -> Vec<Self::ReferenceId> {
        let author_id = self.performer.value();
        vec![ReferenceKey::PerformedBy(PersonId::new(author_id.id()))]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
