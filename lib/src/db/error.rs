use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidUniqueId(String),
    IllegalReference(String, String, String),
    InternalError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUniqueId(id) => f.write_fmt(format_args!("{id}: no such identifier")),
            Self::IllegalReference(from, what, to) => {
                f.write_fmt(format_args!("illegal reference {what} from {from} to {to}"))
            }
            Self::InternalError => f.write_str("internal error"),
        }
    }
}
