use crate::model::property::LinkProperty;

use super::ModelRepr;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct LinkRepr {
    pub kind: String,
    pub url: String,
}

impl ModelRepr for LinkRepr {}

impl From<LinkProperty> for LinkRepr {
    fn from(value: LinkProperty) -> Self {
        LinkRepr {
            kind: value.content.into(),
            url: value.url.to_string(),
        }
    }
}
