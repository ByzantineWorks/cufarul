use super::node::NodeIdentity;
use std::{
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
    result::Result,
    sync::Arc,
};

/// The base trait of a database edge.
///
/// The edges can carry arbitrary data, but since the database does not care
/// about the data, it only requires edges to implement the `EdgeLike` trait.
/// TODO: remove Debug
pub trait EdgeLike: Debug + Send + Sync {}

/// A reference-counted edge data instance.
pub type EdgeRef = Arc<dyn EdgeLike>;

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

/// Trait required for all types implementing reference identities
pub trait ReferenceIdentity<NodeId>: Clone + Display + Hash + Eq + PartialEq {
    type Error: Error;
    fn object(&self) -> NodeId;
    fn reverse(&self, object: NodeId) -> Result<Self, Self::Error>;
}

impl<NodeId, ReferenceId> EdgeId<NodeId, ReferenceId>
where
    NodeId: NodeIdentity,
    ReferenceId: ReferenceIdentity<NodeId>,
{
    pub fn new(subject: NodeId, predicate: ReferenceId) -> Self {
        EdgeId {
            subject: subject,
            predicate: predicate,
        }
    }

    pub fn subject(&self) -> NodeId {
        self.subject.to_owned()
    }

    pub fn forward_predicate(&self) -> ReferenceId {
        self.predicate.to_owned()
    }

    pub fn backward_predicate(&self) -> Result<ReferenceId, ReferenceId::Error> {
        self.predicate.reverse(self.subject.to_owned())
    }

    pub fn object(&self) -> NodeId {
        self.predicate.object()
    }

    pub fn reverse(&self) -> Result<Self, ReferenceId::Error> {
        Ok(EdgeId {
            subject: self.object(),
            predicate: self.backward_predicate()?,
        })
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
        f.write_fmt(format_args!(
            "{} -> {}",
            self.subject(),
            self.forward_predicate()
        ))
    }
}
