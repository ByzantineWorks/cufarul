use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::fs;
use crate::fields::{GenericField, NonEmptyString, TranslatableField};
use crate::error::{Error, Result};

pub trait Model
where
	Self: Sized + DeserializeOwned,
{
	fn validate(&self) -> bool;
	fn from_file(filepath: String) -> Result<Self> {
		let content: String = fs::read_to_string(filepath)?;
		let object: Self = toml::from_str(&content)?;

		if object.validate() {
			return Ok(object);
		} else {
			return Err(Error::Semantic(String::from("validation error")));
		}
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

impl Model for Person {
	fn validate(&self) -> bool {
		if self.name.num_translations() == 0 {
			return false;
		}

		return true;
	}
}
