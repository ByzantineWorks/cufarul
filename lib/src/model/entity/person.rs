use super::property::{LinkProperty, TranslatableProperty};
use super::{Model, ReferenceKey};
use crate::db::NodeLike;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    name: TranslatableProperty,
    about: Option<LinkProperty>,
}

impl Model for Person {
    fn references(&self) -> Vec<ReferenceKey> {
        vec![]
    }
}

impl NodeLike for Person {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
