use crate::db::INode;
use crate::serde::{Error, Result};
use serde::de::DeserializeOwned;
use std::path::PathBuf;

pub trait Model: INode
where
    Self: DeserializeOwned,
{
    fn load(path: PathBuf) -> Result<Self> {
        match std::fs::read_to_string(path.to_owned()) {
            Ok(content) => Ok(toml::from_str(&content)?),
            Err(error) => Err(Error::InternalError(format!(
                "could not read {:?}: {error}",
                path.to_owned()
            ))),
        }
    }
}
