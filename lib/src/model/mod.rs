mod person;

pub use person::Person;

use crate::db::Identify;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum NodeKind {
    Person,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum EdgeKind {
    Author,
}

impl Identify for EdgeKind {
    fn identity(&self) -> String {
        match self {
            EdgeKind::Author => "author".to_owned(),
        }
    }
}

impl From<EdgeKind> for String {
    fn from(value: EdgeKind) -> Self {
        value.identity()
    }
}
