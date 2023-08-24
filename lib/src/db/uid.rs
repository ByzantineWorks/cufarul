use std::hash::Hash;

use super::{edge::Identify, Edge};

pub type UniqueId = String;

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct EdgeHash(String);

impl EdgeHash {
    pub fn incoming<E>(edge: &Edge<E>) -> Self
    where
        E: Into<String> + Hash + Eq + Identify,
    {
        let mut ctx = md5::Context::new();
        ctx.consume(edge.subject());
        ctx.consume(edge.predicate().identity());

        EdgeHash(format!("{:x}", ctx.compute()))
    }

    pub fn outgoing<E>(edge: &Edge<E>) -> Self
    where
        E: Into<String> + Hash + Eq + Identify,
    {
        let mut ctx = md5::Context::new();
        ctx.consume(edge.object());
        ctx.consume(edge.predicate().identity());

        EdgeHash(format!("{:x}", ctx.compute()))
    }
}
