use crate::{
    error::{Error, Result},
    serde::VersionInfo,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct DatabaseSection {
    pub version: VersionInfo,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct DatabaseSpec {
    #[serde(skip)]
    pub root: PathBuf,
    pub database: DatabaseSection,
}

impl TryFrom<PathBuf> for DatabaseSpec {
    type Error = Error;
    fn try_from(value: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(&value)?;
        let mut spec: DatabaseSpec = toml::from_str(&content)?;

        spec.root = value.parent().unwrap().to_path_buf();
        Ok(spec)
    }
}
