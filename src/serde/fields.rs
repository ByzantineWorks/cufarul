use super::{translation::TranslationMap, Lang, NonEmptyString};
use crate::error::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};

pub trait Field<T> {
    fn value(&self, lang: Option<Lang>) -> Result<T>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct GenericField<T> {
    #[serde(flatten)]
    data: T,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TranslatableField {
    #[serde(flatten)]
    data: TranslationMap,

    #[serde(skip)]
    default_lang: Option<Lang>,
}

impl<T> Field<T> for GenericField<T>
where
    T: Clone,
{
    fn value(&self, _lang: Option<Lang>) -> Result<T> {
        Ok(self.data.clone())
    }
}

impl Field<NonEmptyString> for TranslatableField {
    fn value(&self, lang: Option<Lang>) -> Result<NonEmptyString> {
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

        Ok(value.unwrap().to_owned())
    }
}

/*
 * ReferenceField
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ReferenceField<T>
where
    T: Clone,
{
    collection: NonEmptyString,
    id: NonEmptyString,
    target: Option<Rc<T>>,
}

impl<T> Field<Rc<T>> for ReferenceField<T>
where
    T: Clone,
{
    fn value(&self, _: Option<Lang>) -> Result<Rc<T>> {
        match &self.target {
            Some(target) => Ok(target.clone()),
            None => Err(Error::NoValue),
        }
    }
}

impl<T> TryFrom<String> for ReferenceField<T>
where
    T: Clone,
{
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        let regex = Regex::new(r"^@(\w+)/([\w-]+)").unwrap();
        let mut captures = regex.captures_iter(value.trim());
        match captures.next() {
            Some(capture) => {
                let (_, [collection, id]) = capture.extract();

                Ok(ReferenceField {
                    collection: NonEmptyString::try_from(String::from(collection))?,
                    id: NonEmptyString::try_from(String::from(id))?,
                    target: None,
                })
            }
            None => Err(Error::InvalidReferenceString(value)),
        }
    }
}

impl<T> From<ReferenceField<T>> for String
where
    T: Clone,
{
    fn from(value: ReferenceField<T>) -> Self {
        format!("{value}")
    }
}

impl<T> Display for ReferenceField<T>
where
    T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "@{}/{}",
            self.collection.value(),
            self.id.value()
        ))
    }
}
