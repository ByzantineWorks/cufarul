use super::{property::Property, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ReferenceProperty {
    collection: Option<String>,
    id: String,
}

impl TryFrom<String> for ReferenceProperty {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        value
            .strip_prefix("@")
            .ok_or(Error::InvalidReference(value.to_owned()))
            .and_then(|res| {
                Ok(match res.split_once("/") {
                    Some((collection, id)) => ReferenceProperty {
                        collection: Some(collection.to_owned()),
                        id: id.to_owned(),
                    },
                    None => ReferenceProperty {
                        collection: None,
                        id: res.to_owned(),
                    },
                })
            })
    }
}

impl From<ReferenceProperty> for String {
    fn from(value: ReferenceProperty) -> Self {
        match value.collection {
            Some(collection) => format!("@{}/{}", collection, value.id),
            None => format!("@{}", value.id),
        }
    }
}

impl Property<(Option<String>, String)> for ReferenceProperty {
    fn value(&self, _: Option<super::Lang>) -> Option<(Option<String>, String)> {
        Some((self.collection.to_owned(), self.id.to_owned()))
    }
}
