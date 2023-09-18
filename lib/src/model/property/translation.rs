use crate::model::Query;

use super::{Error, Lang, NonEmptyString, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type TranslationMapInternal = HashMap<Lang, NonEmptyString>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "TranslationMapInternal")]
#[serde(into = "TranslationMapInternal")]
pub struct TranslationMap(TranslationMapInternal);

impl TryFrom<TranslationMapInternal> for TranslationMap {
    type Error = Error;
    fn try_from(value: TranslationMapInternal) -> Result<Self> {
        match value.is_empty() {
            true => Err(Error::MissingTranslation),
            false => Ok(TranslationMap(value)),
        }
    }
}

impl From<TranslationMap> for TranslationMapInternal {
    fn from(value: TranslationMap) -> Self {
        value.0
    }
}

impl TranslationMap {
    pub fn translation(&self, lang: &Lang) -> Option<String> {
        self.0.get(&lang).map(|v| v.value().to_owned())
    }

    pub fn any_translation(&self) -> String {
        // Safely unwrap since a translation map contains at least one translation
        self.0.iter().next().unwrap().1.value().to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranslatableProperty {
    #[serde(flatten)]
    data: TranslationMap,

    #[serde(skip)]
    default_lang: Option<Lang>,
}

impl TranslatableProperty {
    pub fn value(&self, lang: Option<Lang>) -> String {
        match lang.or(self.default_lang.to_owned()) {
            Some(lang) => self
                .data
                .translation(&lang)
                .unwrap_or(self.data.any_translation()),
            None => self.data.any_translation(),
        }
    }
}

impl Query for TranslatableProperty {
    fn contains(&self, value: String) -> bool {
        self.data.0.values().any(|t| {
            t.value()
                .to_ascii_lowercase()
                .contains(&value.to_ascii_lowercase())
                || t.value().to_lowercase().contains(&value.to_lowercase())
        })
    }
}
