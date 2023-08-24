use super::UniqueId;
use std::hash::Hash;

pub trait Identify {
    fn identity(&self) -> String;
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Edge<E>
where
    E: Sized + Hash + Eq + Identify,
{
    subject: UniqueId,
    object: UniqueId,
    predicate: E,
}

impl<E> Edge<E>
where
    E: Hash + Eq + Identify,
{
    pub fn new(sub: UniqueId, obj: UniqueId, pred: E) -> Self {
        Edge {
            subject: sub,
            object: obj,
            predicate: pred,
        }
    }

    pub fn subject(&self) -> &UniqueId {
        &self.subject
    }

    pub fn object(&self) -> &UniqueId {
        &self.object
    }

    pub fn predicate(&self) -> &E {
        &self.predicate
    }
}
