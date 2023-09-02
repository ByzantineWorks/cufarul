use std::{
    any::Any,
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};

/// The base trait of a database node.
///
/// The database implementation does not care for the content of the nodes but
/// it requires the nodes to implement the `NodeLike` trait.
/// TODO: remove Debug
pub trait NodeLike: Debug
where
    Self: Any,
{
    type ReferenceId;

    fn references(&self) -> Vec<Self::ReferenceId>;
    fn as_any(&self) -> &dyn Any;
}

/// A reference-counted node.
pub type NodeRef<R> = Rc<dyn NodeLike<ReferenceId = R>>;

/// Wrapper over a node
pub struct Node<NodeId, ReferenceId> {
    id: NodeId,
    data: NodeRef<ReferenceId>,
}

// Trait required for all types implementing node identities
pub trait NodeIdentity: Clone + Display + Hash + Eq + PartialEq {}

impl<NodeId, ReferenceId> Node<NodeId, ReferenceId> {
    pub fn new(id: NodeId, data: NodeRef<ReferenceId>) -> Self {
        Node { id: id, data: data }
    }
}
