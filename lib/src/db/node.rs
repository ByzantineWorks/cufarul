use std::{
    any::Any,
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    sync::Arc,
};

use super::ReferenceIdentity;

/// The base trait of a database node.
///
/// The database implementation does not care for the content of the nodes but
/// it requires the nodes to implement the `NodeLike` trait.
/// TODO: remove Debug
pub trait NodeLike: Debug + Send + Sync + AsAnyArc
where
    Self: Any,
{
    fn as_any(&self) -> &dyn Any;
}

pub trait AsAnyArc {
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: 'static + Send + Sync> AsAnyArc for T {
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}

/// A reference-counted node.
pub type NodeRef = Arc<dyn NodeLike>;

/// Wrapper over a node
#[derive(Debug, Clone)]
pub struct Node<NodeId, ReferenceId>
where
    ReferenceId: ReferenceIdentity<NodeId>,
{
    id: NodeId,
    data: NodeRef,
    references: HashSet<ReferenceId>,
}

// Trait required for all types implementing node identities
pub trait NodeIdentity: Clone + Display + Hash + Eq + PartialEq {}

impl<NodeId, ReferenceId> Node<NodeId, ReferenceId>
where
    NodeId: NodeIdentity,
    ReferenceId: ReferenceIdentity<NodeId>,
{
    pub fn new(id: NodeId, data: NodeRef) -> Self {
        Node {
            id: id,
            data: data,
            references: HashSet::default(),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id.to_owned()
    }

    pub fn data(&self) -> NodeRef {
        self.data.clone()
    }

    pub fn references(&self) -> std::collections::hash_set::Iter<ReferenceId> {
        self.references.iter()
    }

    pub(super) fn push_reference(&mut self, reference: ReferenceId) {
        self.references.insert(reference);
    }
}
