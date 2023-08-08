use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::error::{Error, Result};

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
