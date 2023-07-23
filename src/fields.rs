use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::Error;

pub trait Field <T> {
	fn value(&self, lang: Option <Lang>) -> Result <&T, Error>;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub enum Lang {
	AR,
	EN,
	GR,
	RO,
}

impl TryFrom <String> for Lang {
	type Error = Error;
	fn try_from(value: String) -> Result<Self, self::Error> {
		return match value.trim() {
			"ar" => Ok(Lang::AR),
			"en" => Ok(Lang::EN),
			"gr" => Ok(Lang::GR),
			"ro" => Ok(Lang::RO),
			_ => Err(Error::LanguageNotSupported)
		};
	}
}

impl Into <String> for Lang {
	fn into(self) -> String {
		return match self {
			Lang::AR => String::from("ar"),
			Lang::EN => String::from("en"),
			Lang::GR => String::from("gr"),
			Lang::RO => String::from("ro"),
		}
	}
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[serde(transparent)]
pub struct GenericField <T> {
	#[serde(flatten)]
	data: T,
}
#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct TranslatableField {
	#[serde(flatten)]
	data: HashMap <Lang, String>,

	#[serde(skip)]
	default_lang: Option <Lang>,
}

impl TranslatableField {
	pub fn num_translations (&self) -> usize {
		self.data.len()
	}
}

impl <T> Field <T> for GenericField <T> {
	fn value(&self, _lang: Option <Lang>) -> Result <&T, Error> {
			Ok(&self.data)
	}
}

impl Field<String> for TranslatableField {
	fn value(&self, lang: Option <Lang>) -> Result <&String, Error> {
		let mut language = lang;
		if language.is_none() && self.default_lang.is_some() {
			language = self.default_lang.clone()
		}

		if language.is_some() {
			let value = self.data.get(language.as_ref().unwrap());
			if value.is_some() {
				return Ok(value.unwrap())
			} else {
				return Err(Error::ValueNotFound)
			}
		}

		Err(Error::ValueNotFound)
	}
}
