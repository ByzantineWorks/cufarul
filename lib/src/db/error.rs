use super::UniqueId;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidUniqueId(UniqueId),
    InternalError,
}
