use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoRepositoryFound,
    UnsupportedVersion(u8),
    MissingCollection(String),
    InvalidReference(String),
    IoError(std::io::Error),
    DeError(toml::de::Error),
    NoData(String),
    InternalError(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoRepositoryFound => f.write_str("no repository found"),
            Self::UnsupportedVersion(version) => {
                f.write_fmt(format_args!("unsupported repository version: {version}"))
            }
            Self::InvalidReference(spec) => {
                f.write_fmt(format_args!("could not find reference: {spec}"))
            }
            Self::IoError(error) => f.write_fmt(format_args!("could not read repository: {error}")),
            Self::DeError(error) => {
                f.write_fmt(format_args!("invalid repository configuration: {error}"))
            }
            Self::MissingCollection(collection) => {
                f.write_fmt(format_args!("missing collection directory: {collection}"))
            }
            Self::NoData(id) => f.write_fmt(format_args!("no data for id: {id}")),
            Self::InternalError(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::DeError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
