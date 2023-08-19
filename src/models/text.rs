use super::{Model, Person};
use crate::serde::{ReferenceField, TranslatableField};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    title: TranslatableField,
    author: ReferenceField<Person>,
}

impl Model for Text {}
