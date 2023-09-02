use super::{
    error::{Error, Result},
    REPOSITORY_CONFIG_FILE,
};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize)]
struct DatabaseSection {
    version: u8,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepositorySpec {
    #[serde(skip)]
    root: PathBuf,

    database: DatabaseSection,
}

impl RepositorySpec {
    pub fn from_path_recursive(path: &Path) -> Result<Self> {
        let mut current = std::fs::canonicalize(path)?;
        loop {
            match (
                current.join(REPOSITORY_CONFIG_FILE).exists(),
                current.parent(),
            ) {
                (true, _) => return Self::from_path(&current),
                (_, Some(path)) => current = path.canonicalize()?,
                (_, _) => return Err(Error::NoRepositoryFound),
            }
        }
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let root = path.canonicalize()?;
        let config = root.join(REPOSITORY_CONFIG_FILE);

        config
            .exists()
            .then_some(std::fs::read_to_string(config)?)
            .ok_or(Error::NoRepositoryFound)
            .and_then(|content| Ok(toml::from_str::<Self>(&content)?))
            .and_then(|mut spec| {
                spec.root = root;
                Ok(spec)
            })
    }

    pub fn version(&self) -> u8 {
        self.database.version
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}
