use crate::error::Result;
use erased_serde::serialize_trait_object;
use serde::de::DeserializeOwned;
use std::{fs, path::PathBuf};

pub trait Model: erased_serde::Serialize {}
serialize_trait_object!(Model);

mod person;
mod text;

pub fn from_file<T>(filepath: PathBuf) -> Result<T>
where
    T: Model + DeserializeOwned,
{
    let content: String = fs::read_to_string(filepath)?;
    let object: T = toml::from_str(&content)?;

    Ok(object)
}

pub use person::Person;
pub use text::Text;
