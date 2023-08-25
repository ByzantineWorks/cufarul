use std::fmt::Display;

use super::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub enum Lang {
    AR,
    EN,
    GR,
    RO,
}

impl TryFrom<String> for Lang {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        match value.trim() {
            "ar" => Ok(Lang::AR),
            "en" => Ok(Lang::EN),
            "gr" => Ok(Lang::GR),
            "ro" => Ok(Lang::RO),
            code => Err(Error::UnsupportedLanguage(code.to_owned())),
        }
    }
}

impl From<Lang> for String {
    fn from(value: Lang) -> Self {
        match value {
            Lang::AR => "ar".to_owned(),
            Lang::EN => "en".to_owned(),
            Lang::GR => "gr".to_owned(),
            Lang::RO => "ro".to_owned(),
        }
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from(self.to_owned()).as_str())
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::EN
    }
}
