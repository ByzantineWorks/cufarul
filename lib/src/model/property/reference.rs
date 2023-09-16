use super::{Error, Result};
use crate::model::CollectionKey;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ReferenceProperty(CollectionKey);

impl TryFrom<String> for ReferenceProperty {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        // We unwrap with confidence since the regex is correct and below the
        // size limit.
        Regex::new(r"\A@(\w+)/([\w-]+)\z")
            .unwrap()
            .captures(&value)
            .ok_or(Error::InvalidReference(value.to_owned()))
            .and_then(|capture| {
                let (_, [collection, id]) = capture.extract();
                Ok(ReferenceProperty(CollectionKey::from_collection_and_id(
                    collection.to_owned(),
                    id.to_owned(),
                )?))
            })
    }
}

impl From<ReferenceProperty> for String {
    fn from(value: ReferenceProperty) -> Self {
        value.0.to_string()
    }
}

impl ReferenceProperty {
    pub fn value(&self) -> CollectionKey {
        self.0.to_owned()
    }
}
