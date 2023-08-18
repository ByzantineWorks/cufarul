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

/*
 * VersionInfo - representation of a version number
 */
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub struct VersionInfo {
    major: u8,
}

impl TryFrom<u8> for VersionInfo {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self> {
        Ok(VersionInfo { major: value })
    }
}

impl From<VersionInfo> for u8 {
    fn from(value: VersionInfo) -> Self {
        value.major
    }
}

impl VersionInfo {
    pub fn major(&self) -> u8 {
        self.major
    }
}
