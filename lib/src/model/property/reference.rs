use super::{Error, Result};
use crate::model::CollectionKey;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ReferenceProperty(CollectionKey, Option<String>);

impl TryFrom<String> for ReferenceProperty {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        // We unwrap with confidence since the regex is correct and below the
        // size limit.
        Regex::new(r"\A@(\w+)/([\w-]+)(?:\[([\w-]+)\])?\z")
            .unwrap()
            .captures(&value)
            .ok_or(Error::InvalidReference(value.to_owned()))
            .and_then(|capture| {
                let collection = capture.get(1).unwrap().as_str();
                let id = capture.get(2).unwrap().as_str();
                let variant = capture.get(3).and_then(|m| Some(m.as_str().to_owned()));
                Ok(ReferenceProperty(
                    CollectionKey::from_collection_and_id(collection.to_owned(), id.to_owned())?,
                    variant,
                ))
            })
    }
}

impl From<ReferenceProperty> for String {
    fn from(value: ReferenceProperty) -> Self {
        format!("{}[{}]", value.0.to_string(), value.1.unwrap_or_default())
    }
}

impl ReferenceProperty {
    pub fn value(&self) -> CollectionKey {
        self.0.to_owned()
    }

    pub fn variant(&self) -> Option<String> {
        self.1.to_owned()
    }
}
