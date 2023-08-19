use super::Model;
use crate::serde::{GenericField, NonEmptyString, TranslatableField};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
pub struct Person {
    name: TranslatableField,
    link: GenericField<NonEmptyString>,
}

impl Model for Person {}
