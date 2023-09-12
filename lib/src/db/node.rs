use std::{
    any::Any,
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};

use super::ReferenceIdentity;

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
#[derive(Debug, Clone)]
pub struct Node<NodeId, ReferenceId>
where
    ReferenceId: ReferenceIdentity<NodeId>,
{
    id: NodeId,
    data: NodeRef<ReferenceId>,
    references: HashSet<ReferenceId>,
}

// Trait required for all types implementing node identities
pub trait NodeIdentity: Clone + Display + Hash + Eq + PartialEq {}

impl<NodeId, ReferenceId> Node<NodeId, ReferenceId>
where
    NodeId: NodeIdentity,
    ReferenceId: ReferenceIdentity<NodeId>,
{
    pub fn new(id: NodeId, data: NodeRef<ReferenceId>) -> Self {
        Node {
            id: id,
            data: data,
            references: HashSet::default(),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id.to_owned()
    }

    pub fn data(&self) -> NodeRef<ReferenceId> {
        self.data.clone()
    }

    pub fn references(&self) -> std::collections::hash_set::Iter<ReferenceId> {
        self.references.iter()
    }

    pub(super) fn push_reference(&mut self, reference: ReferenceId) {
        self.references.insert(reference);
    }
}
