use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;
use crate::fields::{GenericField, NonEmptyString, TranslatableField};
use crate::error::Result;

pub trait Model : erased_serde::Serialize {}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
pub struct Person {
	name: TranslatableField,
	link: GenericField <NonEmptyString>,
}

impl Model for Person {}

serialize_trait_object!(Model);

pub fn from_file <T> (filepath: PathBuf) -> Result<T>
where T: Model + DeserializeOwned {
	let content: String = fs::read_to_string(filepath)?;
	let object: T = toml::from_str(&content)?;

	Ok(object)
}
