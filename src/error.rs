use serde::{de, ser};
use std::fmt::Display;

use crate::serde::Lang;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /* Error variants for serde errors */
    DeserializationError(String),
    SerializationError(String),

    /* Error variants for IO errors */
    IOError(std::io::Error),

    /* Error variants for semantic and syntax errors */
    LanguageNotSupported(String),
    NoTranslation,
    NoValue,
    Semantic(String),
    TranslationUnavailable(Lang),

    /* Error variants for database management */
    NoDatabase,
    InvalidCollection(String),
    UnsupportedDatabaseVersion(u8),
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::SerializationError(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DeserializationError(e) | Error::SerializationError(e) => f.write_str(e),
            Error::IOError(e) => f.write_str(e.to_string().as_str()),
            Error::LanguageNotSupported(lang) => {
                f.write_str(format!("{lang}: language not supported").as_str())
            }
            Error::NoValue => f.write_str("field has no value"),
            Error::NoTranslation => f.write_str("no translation given"),
            Error::Semantic(e) => f.write_str(e),
            Error::TranslationUnavailable(lang) => {
                f.write_str(format!("no translation available for {lang}").as_str())
            }
            Error::NoDatabase => f.write_str("no database found"),
            Error::InvalidCollection(c) => f.write_str(format!("{c}: invalid collection").as_str()),
            Error::UnsupportedDatabaseVersion(v) => {
                f.write_str(format!("unsupported database version {v}").as_str())
            }
        }
    }
}

impl std::error::Error for Error {}

/* Error conversions */
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::DeserializationError(value.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Error::SerializationError(value.to_string())
    }
}
