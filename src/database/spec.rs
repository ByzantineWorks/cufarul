use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct DatabaseSection {
    collections: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct DatabaseSpec {
    #[serde(skip)]
    root: PathBuf,
    database: DatabaseSection,
}

impl TryFrom<PathBuf> for DatabaseSpec {
    type Error = crate::error::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(&value)?;
        let mut spec: DatabaseSpec = toml::from_str(&content)?;

        spec.root = value.parent().unwrap().to_path_buf();
        Ok(spec)
    }
}

impl DatabaseSpec {
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn collections(&self) -> &Vec<String> {
        &self.database.collections
    }
}
