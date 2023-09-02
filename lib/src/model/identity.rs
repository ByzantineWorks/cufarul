use std::fmt::Display;

// use crate::db::NodeIdentity;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PersonId(String);

impl PersonId {
    pub fn new(id: String) -> Self {
        PersonId(id)
    }
}

impl Display for PersonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositionId(String);

impl CompositionId {
    pub fn new(id: String) -> Self {
        CompositionId(id)
    }
}

impl Display for CompositionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
