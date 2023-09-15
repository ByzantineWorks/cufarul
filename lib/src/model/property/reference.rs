use super::{Error, Property, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ReferenceProperty {
    collection: String,
    id: String,
}

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

                Ok(ReferenceProperty {
                    collection: collection.to_owned(),
                    id: id.to_owned(),
                })
            })
    }
}

impl From<ReferenceProperty> for String {
    fn from(value: ReferenceProperty) -> Self {
        format!("@{}/{}", value.collection, value.id)
    }
}

impl Property<(String, String)> for ReferenceProperty {
    fn value(&self, _: Option<super::Lang>) -> Option<(String, String)> {
        Some((self.collection.to_owned(), self.id.to_owned()))
    }
}
