use super::Model;
use crate::{
    db::NodeLike,
    model::{
        identity::PersonId,
        property::{ReferenceProperty, TranslatableProperty},
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

impl Model for Publication {
    fn references(&self) -> Vec<ReferenceKey> {
        if let Some(author) = &self.author {
            let author_id = author.value();
            return vec![ReferenceKey::AuthoredBy(PersonId::new(author_id.id()))];
        };

        vec![]
    }
}

impl NodeLike for Publication {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
