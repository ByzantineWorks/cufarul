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
    pub fn translation(&self, lang: Lang) -> Option<&NonEmptyString> {
        self.0.get(&lang)
    }

    pub fn any_translation(&self) -> Option<&NonEmptyString> {
        self.0.iter().next().and_then(|(_, res)| Some(res))
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
    pub fn value(&self, lang: Option<Lang>) -> Option<NonEmptyString> {
        match lang.or(self.default_lang.to_owned()) {
            Some(lang) => self.data.translation(lang),
            None => self.data.any_translation(),
        }
        .and_then(|res| Some(res.to_owned()))
    }
}
