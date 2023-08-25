use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnsupportedLanguage(String),
    DeserializationError(String),
    InternalError(String),
    MissingTranslation,
    MissingValue,
    InvalidReference(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedLanguage(code) => {
                f.write_fmt(format_args!("{code}: language not supported"))
            }
            Error::DeserializationError(msg) => {
                f.write_fmt(format_args!("deserialization error: {msg}"))
            }
            Error::InternalError(msg) => f.write_fmt(format_args!("internal error: {msg}")),
            Error::MissingTranslation => f.write_str("no translation given"),
            Error::MissingValue => f.write_str("propery cannot be empty"),
            Error::InvalidReference(reference) => {
                f.write_fmt(format_args!("{reference}: invalid reference"))
            }
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::DeserializationError(value.to_string())
    }
}
