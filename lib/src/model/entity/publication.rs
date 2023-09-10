use super::Model;
use crate::{
    db::NodeLike,
    model::{
        identity::PersonId,
        property::{Property, ReferenceProperty, TranslatableProperty},
        ReferenceKey,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    name: TranslatableProperty,
    author: ReferenceProperty,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub into: Publication,
    pub page: u16,
}

impl Model for Publication {}
impl NodeLike for Publication {
    type ReferenceId = ReferenceKey;

    fn references(&self) -> Vec<Self::ReferenceId> {
        let (_, author_id) = self.author.value(None).unwrap();
        vec![ReferenceKey::PublishedBy(PersonId::new(author_id))]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
