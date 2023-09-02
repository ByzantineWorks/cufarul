use std::{
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};

use super::node::NodeIdentity;

/// The base trait of a database edge.
///
/// The edges can carry arbitrary data, but since the database does not care
/// about the data, it only requires edges to implement the `EdgeLike` trait.
/// TODO: remove Debug
pub trait EdgeLike: Debug {}

/// A reference-counted edge data instance.
pub type EdgeRef = Rc<dyn EdgeLike>;

/// Wrapper over an edge
pub struct Edge<NodeId, ReferenceId> {
    id: EdgeId<NodeId, ReferenceId>,
    data: EdgeRef,
}

#[derive(Clone)]
/// Full specification of an edge (from, what, to).
pub struct EdgeId<NodeId, ReferenceId> {
    subject: NodeId,
    predicate: ReferenceId,
}

// Trait required for all types implementing reference identities
pub trait ReferenceIdentity<NodeId>: Clone + Display + Hash + Eq + PartialEq {
    fn object(&self) -> NodeId;
}

impl<NodeId, ReferenceId> EdgeId<NodeId, ReferenceId> {
    pub fn new(subject: NodeId, predicate: ReferenceId) -> Self {
        EdgeId {
            subject: subject,
            predicate: predicate,
        }
    }

    pub fn subject(&self) -> &NodeId {
        &self.subject
    }

    pub fn predicate(&self) -> &ReferenceId {
        &self.predicate
    }
}

impl<NodeId, ReferenceId> Edge<NodeId, ReferenceId> {
    pub fn new(id: EdgeId<NodeId, ReferenceId>, data: EdgeRef) -> Self {
        Edge { id: id, data: data }
    }
}

impl<NodeId, ReferenceId> Display for EdgeId<NodeId, ReferenceId>
where
    NodeId: NodeIdentity,
    ReferenceId: ReferenceIdentity<NodeId>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -> {}", self.subject(), self.predicate()))
    }
}
