use std::{collections::HashMap, fmt::Display, hash::Hash};
use serde::{Deserialize, Serialize};
use crate::error::{Error, Result};

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct NonEmptyString(String);

impl TryFrom<String> for NonEmptyString {
	type Error = Error;
	fn try_from(value: String) -> Result<Self> {
		match value.is_empty() {
			true => Err(Error::NoValue),
			false => Ok(NonEmptyString(value))
		}
	}
}

impl From<NonEmptyString> for String {
	fn from(value: NonEmptyString) -> Self {
		value.0
	}
}


#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(try_from = "HashMap<Lang, NonEmptyString>")]
#[serde(into = "HashMap<Lang, NonEmptyString>")]
pub struct TranslationMap(HashMap<Lang, NonEmptyString>);

impl TryFrom<HashMap<Lang, NonEmptyString>> for TranslationMap {
	type Error = Error;
	fn try_from(value: HashMap<Lang, NonEmptyString>) -> Result<Self> {
		match value.is_empty() {
			true => Err(Error::NoTranslation),
			false => Ok(TranslationMap(value))
		}
	}
}

impl From<TranslationMap> for HashMap<Lang, NonEmptyString> {
	fn from(value: TranslationMap) -> Self {
			value.0
	}
}


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
	data: TranslationMap,

	#[serde(skip)]
	default_lang: Option <Lang>,
}

impl <T> Field <T> for GenericField <T> {
	fn value(&self, _lang: Option <Lang>) -> Result <&T> {
			Ok(&self.data)
	}
}

impl Field<NonEmptyString> for TranslatableField {
	fn value(&self, lang: Option <Lang>) -> Result <&NonEmptyString> {
		let mut language = lang;
		if language.is_none() && self.default_lang.is_some() {
			language = self.default_lang.clone()
		}

		if language.is_none() {
			return Err(Error::NoValue);
		}

		let value = self.data.0.get(language.as_ref().unwrap());
		if value.is_none() {
			return Err(Error::TranslationUnavailable(language.unwrap()));
		}

		Ok(value.unwrap())
	}
}
