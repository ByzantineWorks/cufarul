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
    author: Option<ReferenceProperty>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub into: ReferenceProperty,
    pub page: u16,
}

impl Model for Publication {}
impl NodeLike for Publication {
    type ReferenceId = ReferenceKey;

    fn references(&self) -> Vec<Self::ReferenceId> {
        if let Some(author) = &self.author {
            let (_, author_id) = author.value(None).unwrap();
            return vec![ReferenceKey::PublishedBy(PersonId::new(author_id))];
        };

        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
