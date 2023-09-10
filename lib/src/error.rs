use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RepositoryError(crate::repo::Error),
    DatabaseError(crate::db::Error),
    ModelError(crate::model::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RepositoryError(error) => f.write_fmt(format_args!("repository error: {error}")),
            Error::DatabaseError(error) => f.write_fmt(format_args!("database error: {error}")),
            Error::ModelError(error) => f.write_fmt(format_args!("model error: {error}")),
        }
    }
}

impl From<crate::repo::Error> for Error {
    fn from(value: crate::repo::Error) -> Self {
        Error::RepositoryError(value)
    }
}

impl From<crate::db::Error> for Error {
    fn from(value: crate::db::Error) -> Self {
        Error::DatabaseError(value)
    }
}

impl From<crate::model::Error> for Error {
    fn from(value: crate::model::Error) -> Self {
        Error::ModelError(value)
    }
}
