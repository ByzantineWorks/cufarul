use super::CollectionKey;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IncompatibleKeys(CollectionKey, String),
    UnsupportedLanguage(String),
    DeserializationError(String),
    InternalError(String),
    MissingTranslation,
    MissingValue,
    InvalidReference(String),
    InvalidExternalLink(String, String),
    InvalidContribution(String),
    InvalidCollectionKey(String),
    InvalidMode(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleKeys(k1, k2) => {
                f.write_fmt(format_args!("incompatible keys: expected {k2} found {k1}"))
            }
            Self::UnsupportedLanguage(code) => {
                f.write_fmt(format_args!("{code}: language not supported"))
            }
            Self::DeserializationError(msg) => {
                f.write_fmt(format_args!("deserialization error: {msg}"))
            }
            Self::InternalError(msg) => f.write_fmt(format_args!("internal error: {msg}")),
            Self::MissingTranslation => f.write_str("no translation given"),
            Self::MissingValue => f.write_str("propery cannot be empty"),
            Self::InvalidReference(reference) => {
                f.write_fmt(format_args!("{reference}: invalid reference"))
            }
            Self::InvalidExternalLink(url, err) => {
                f.write_fmt(format_args!("invalid external link: {url}: {err}"))
            }
            Self::InvalidContribution(contrib) => {
                f.write_fmt(format_args!("invalid contribution: {contrib}"))
            }
            Self::InvalidCollectionKey(key) => {
                f.write_fmt(format_args!("{key}: unsupported collection key"))
            }
            Self::InvalidMode(mode) => f.write_fmt(format_args!(
                "invalid mode: {mode} (expected mode be between 1 and 8)"
            )),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::DeserializationError(msg.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::DeserializationError(value.to_string())
    }
}
