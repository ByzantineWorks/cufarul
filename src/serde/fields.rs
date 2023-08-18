use super::{translation::TranslationMap, Lang, NonEmptyString};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

pub trait Field<T> {
    fn value(&self, lang: Option<Lang>) -> Result<&T>;
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct GenericField<T> {
    #[serde(flatten)]
    data: T,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TranslatableField {
    #[serde(flatten)]
    data: TranslationMap,

    #[serde(skip)]
    default_lang: Option<Lang>,
}

impl<T> Field<T> for GenericField<T> {
    fn value(&self, _lang: Option<Lang>) -> Result<&T> {
        Ok(&self.data)
    }
}

impl Field<NonEmptyString> for TranslatableField {
    fn value(&self, lang: Option<Lang>) -> Result<&NonEmptyString> {
        let mut language = lang;
        if language.is_none() && self.default_lang.is_some() {
            language = self.default_lang.clone()
        }

        if language.is_none() {
            return Err(Error::NoValue);
        }

        let value = self.data.translation(language.as_ref().unwrap());
        if value.is_none() {
            return Err(Error::TranslationUnavailable(language.unwrap()));
        }

        Ok(value.unwrap())
    }
}
