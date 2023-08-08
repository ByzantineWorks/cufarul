use super::{Lang, NonEmptyString};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "HashMap<Lang, NonEmptyString>")]
#[serde(into = "HashMap<Lang, NonEmptyString>")]
pub struct TranslationMap(HashMap<Lang, NonEmptyString>);

impl TryFrom<HashMap<Lang, NonEmptyString>> for TranslationMap {
    type Error = Error;
    fn try_from(value: HashMap<Lang, NonEmptyString>) -> Result<Self> {
        match value.is_empty() {
            true => Err(Error::NoTranslation),
            false => Ok(TranslationMap(value)),
        }
    }
}

impl From<TranslationMap> for HashMap<Lang, NonEmptyString> {
    fn from(value: TranslationMap) -> Self {
        value.0
    }
}

impl TranslationMap {
    pub fn translation(&self, lang: &super::Lang) -> Option<&NonEmptyString> {
        self.0.get(lang)
    }
}
