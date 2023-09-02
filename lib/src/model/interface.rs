use super::ReferenceKey;
use crate::db::NodeLike;
use crate::serde::{Error, Result};
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::rc::Rc;

pub trait Model: NodeLike<ReferenceId = ReferenceKey> + erased_serde::Serialize {}
erased_serde::serialize_trait_object!(Model);

pub fn from_file<T>(path: PathBuf) -> Result<Rc<T>>
where
    T: Model + DeserializeOwned,
{
    match std::fs::read_to_string(path.to_owned()) {
        Ok(content) => Ok(Rc::new(toml::from_str::<T>(&content)?)),
        Err(error) => Err(Error::InternalError(format!(
            "could not read {:?}: {error}",
            path.to_owned()
        ))),
    }
}
