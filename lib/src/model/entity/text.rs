use super::{Model, Query};
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
pub struct Text {
    pub name: TranslatableProperty,
    pub author: Option<ReferenceProperty>,
}

impl Model for Text {
    fn references(&self) -> Vec<ReferenceKey> {
        if let Some(author) = &self.author {
            let author_id = author.value();
            return vec![ReferenceKey::WrittenBy(PersonId::new(author_id.key()))];
        }

        vec![]
    }
}

impl NodeLike for Text {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Query for Text {
    fn contains(&self, value: String) -> bool {
        self.name.contains(value)
    }
}
