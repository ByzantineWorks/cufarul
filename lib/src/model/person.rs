use super::Model;
use crate::{
    db::INode,
    serde::{GenericProperty, NonEmptyString, TranslatableProperty},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    something: GenericProperty<NonEmptyString>,
    name: TranslatableProperty,
}

impl INode for Person {}
impl Model for Person {}
