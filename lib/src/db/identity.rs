use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferenceId<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity + Allowed<NodeId>,
{
    node: NodeId,
    edge: EdgeId,
}

impl<NodeId, EdgeId> ReferenceId<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity + Allowed<NodeId>,
{
    pub fn new(node: NodeId, edge: EdgeId) -> Self {
        ReferenceId {
            node: node,
            edge: edge,
        }
    }
}

pub trait Identity:
    Clone + Debug + Hash + Ord + PartialOrd + Eq + PartialEq + Into<String> + Display
{
}

pub trait Allowed<NodeId> {
    fn is_allowed(&self, _from: &NodeId, _to: &NodeId) -> bool {
        true
    }
}
