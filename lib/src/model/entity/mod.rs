mod composition;
mod person;

pub use composition::Composition;
pub use person::Person;

use super::error;
use super::identity;
use super::property;
use super::serde;
use super::ReferenceKey;
use crate::db::NodeLike;

pub trait Model: NodeLike<ReferenceId = ReferenceKey> + erased_serde::Serialize {}
erased_serde::serialize_trait_object!(Model);

pub fn from_file<T>(path: std::path::PathBuf) -> error::Result<std::rc::Rc<T>>
where
    T: Model + ::serde::de::DeserializeOwned,
{
    match std::fs::read_to_string(path.to_owned()) {
        Ok(content) => Ok(std::rc::Rc::new(toml::from_str::<T>(&content)?)),
        Err(error) => Err(error::Error::InternalError(format!(
            "could not read {:?}: {error}",
            path.to_owned()
        ))),
    }
}
