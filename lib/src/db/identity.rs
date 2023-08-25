use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferenceId<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity,
{
    node: NodeId,
    edge: EdgeId,
}

impl<NodeId, EdgeId> ReferenceId<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity,
{
    pub fn new(node: NodeId, edge: EdgeId) -> Self {
        ReferenceId {
            node: node,
            edge: edge,
        }
    }
}

pub trait Identity:
    Clone + Debug + Hash + Ord + PartialOrd + Eq + PartialEq + Into<String>
{
}
