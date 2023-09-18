mod composition;
mod performance;
mod person;
mod publication;
mod taxonomy;
mod text;

pub mod ser;

use std::sync::Arc;

pub use composition::Composition;
pub use performance::Performance;
pub use person::Person;
pub use publication::{Publication, Reference};
pub use taxonomy::Taxonomy;
pub use text::Text;

use super::error;
use super::identity;
use super::property;
use super::ReferenceKey;
use crate::db::NodeLike;

pub trait Model: NodeLike
where
    Self: Query,
{
    fn references(&self) -> Vec<ReferenceKey>;
}

pub trait Query {
    fn contains(&self, value: String) -> bool;
}

pub fn from_file<T>(path: std::path::PathBuf) -> error::Result<Arc<T>>
where
    T: Model + ::serde::de::DeserializeOwned,
{
    match std::fs::read_to_string(path.to_owned()) {
        Ok(content) => Ok(Arc::new(toml::from_str::<T>(&content)?)),
        Err(error) => Err(error::Error::InternalError(format!(
            "could not read {:?}: {error}",
            path.to_owned()
        ))),
    }
}

pub fn into_traits<T>(data: Arc<T>) -> (Arc<dyn NodeLike>, Arc<dyn Model>)
where
    T: Model,
{
    (data.clone(), data.clone())
}
