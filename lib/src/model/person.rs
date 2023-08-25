use super::Model;
use crate::{
    db::INode,
    serde::{GenericProperty, TranslatableProperty},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    something: GenericProperty<String>,
    name: TranslatableProperty,
}

impl INode for Person {}
impl Model for Person {}
