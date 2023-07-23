use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs;
use crate::fields::GenericField;
use crate::fields::TranslatableField;
use crate::error::Error;

pub trait Model
where
	Self: Sized + DeserializeOwned,
{
	fn validate(&self) -> bool;
	fn from_file(filepath: String) -> Result<Self, Box<dyn std::error::Error>> {
		let content: String = fs::read_to_string(filepath)?;
		let object: Self =toml::from_str(&content)?;

		if object.validate() {
			return Ok(object);
		} else {
			return Err(Box::new(Error::LanguageNotSupported));
		}
	}
}

#[derive(Debug)]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Person {
	name: TranslatableField,
	link: GenericField <String>,
}

impl Model for Person {
	fn validate(&self) -> bool {
		if self.name.num_translations() == 0 {
			return false;
		}

		return true;
	}
}
