use super::{Model, ReferenceKey};
use crate::{
    db::NodeLike,
    model::PersonId,
    serde::{Property, ReferenceProperty},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Composition {
    author: ReferenceProperty,
}

impl Model for Composition {}
impl NodeLike for Composition {
    type ReferenceId = ReferenceKey;

    fn references(&self) -> Vec<Self::ReferenceId> {
        let (_, author_id) = self.author.value(None).unwrap();
        vec![ReferenceKey::AuthoredBy(PersonId::new(author_id))]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
