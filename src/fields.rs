use std::{collections::HashMap, fmt::Display};
use serde::{Deserialize, Serialize};
use crate::error::{Error, Result};

pub trait Field <T> {
	fn value(&self, lang: Option <Lang>) -> Result <&T>;
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
	fn try_from(value: String) -> Result<Self> {
		return match value.trim() {
			"ar" => Ok(Lang::AR),
			"en" => Ok(Lang::EN),
			"gr" => Ok(Lang::GR),
			"ro" => Ok(Lang::RO),
			code => Err(Error::LanguageNotSupported(String::from(code)))
		};
	}
}

impl From<Lang> for String {
	fn from(value: Lang) -> Self {
		match value {
			Lang::AR => String::from("ar"),
			Lang::EN => String::from("en"),
			Lang::GR => String::from("gr"),
			Lang::RO => String::from("ro"),
		}
	}
}

impl Display for Lang {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(String::from(self.to_owned()).as_str())
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
	fn value(&self, _lang: Option <Lang>) -> Result <&T> {
			Ok(&self.data)
	}
}

impl Field<String> for TranslatableField {
	fn value(&self, lang: Option <Lang>) -> Result <&String> {
		let mut language = lang;
		if language.is_none() && self.default_lang.is_some() {
			language = self.default_lang.clone()
		}

		if language.is_none() {
			return Err(Error::NoValue);
		}

		let value = self.data.get(language.as_ref().unwrap());
		if value.is_none() {
			return Err(Error::TranslationUnavailable(language.unwrap()));
		}

		Ok(value.unwrap())
	}
}
