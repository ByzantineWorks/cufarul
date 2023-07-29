use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;
use crate::fields::{GenericField, NonEmptyString, TranslatableField};
use crate::error::Result;

pub trait Model
where
	Self: Sized + DeserializeOwned,
{
	fn from_file(filepath: PathBuf) -> Result<Self> {
		let content: String = fs::read_to_string(filepath)?;
		let object: Self = toml::from_str(&content)?;

		Ok(object)
	}
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
pub struct Person {
	name: TranslatableField,
	link: GenericField <NonEmptyString>,
}

impl Model for Person {}
