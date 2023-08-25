use super::{property::Property, Error, Lang, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type TranslationMapInternal = HashMap<Lang, String>;

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
    pub fn translation(&self, lang: Lang) -> Option<&String> {
        self.0.get(&lang)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranslatableProperty {
    #[serde(flatten)]
    data: TranslationMap,

    #[serde(skip)]
    default_lang: Option<Lang>,
}

impl Property<String> for TranslatableProperty {
    fn value(&self, lang: Option<Lang>) -> Option<String> {
        let language = lang.or(self.default_lang.to_owned()).unwrap_or_default();

        self.data
            .translation(language)
            .and_then(|res| Some(res.to_owned()))
    }
}
