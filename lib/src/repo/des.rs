use crate::repo::{Error, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

const REPOSITORY_CONFIG_FILE: &str = ".cufarul";

#[derive(Debug, Deserialize)]
struct DatabaseSection {
    version: u8,
}

#[derive(Debug, Deserialize)]
pub struct RepositorySpec {
    #[serde(skip)]
    root: PathBuf,

    database: DatabaseSection,
}

impl TryFrom<PathBuf> for RepositorySpec {
    type Error = Error;
    fn try_from(value: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(&value)?;
        let mut spec: RepositorySpec = toml::from_str(&content)?;

        spec.root = value.parent().unwrap().to_path_buf();
        Ok(spec)
    }
}

impl RepositorySpec {
    pub fn from_path(path: &Path) -> Result<RepositorySpec> {
        let mut current_path = std::fs::canonicalize(path)?;

        while !current_path.join(REPOSITORY_CONFIG_FILE).exists() {
            current_path = match current_path.parent() {
                Some(p) => p.canonicalize()?,
                None => return Err(Error::NoRepositoryFound),
            };
        }

        RepositorySpec::try_from(current_path.join(REPOSITORY_CONFIG_FILE))
    }
}

impl RepositorySpec {
    pub fn version(&self) -> u8 {
        self.database.version
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}
