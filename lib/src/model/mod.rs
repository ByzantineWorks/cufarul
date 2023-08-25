mod person;

pub use person::Person;

use crate::db::Identity;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeKind {
    Person(String),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdgeKind {
    Author,
}

impl Identity for NodeKind {}
impl Identity for EdgeKind {}

impl From<NodeKind> for String {
    fn from(value: NodeKind) -> Self {
        match value {
            NodeKind::Person(_) => "person".to_owned(),
        }
    }
}

impl From<EdgeKind> for String {
    fn from(value: EdgeKind) -> Self {
        match value {
            EdgeKind::Author => "author".to_owned(),
        }
    }
}
