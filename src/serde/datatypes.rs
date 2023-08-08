use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct NonEmptyString(String);

impl TryFrom<String> for NonEmptyString {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        match value.is_empty() {
            true => Err(Error::NoValue),
            false => Ok(NonEmptyString(value)),
        }
    }
}

impl From<NonEmptyString> for String {
    fn from(value: NonEmptyString) -> Self {
        value.0
    }
}

impl NonEmptyString {
    pub fn value(&self) -> &String {
        &self.0
    }
}
