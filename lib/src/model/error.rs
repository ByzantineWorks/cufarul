use super::CollectionKey;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IncompatibleKeys(CollectionKey, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleKeys(k1, k2) => {
                f.write_fmt(format_args!("incompatible keys: expected {k2} found {k1}"))
            }
        }
    }
}
