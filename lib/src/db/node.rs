use std::{fmt::Debug, rc::Rc};

use super::ReferenceList;

#[derive(Debug)]
pub struct Node<NodeId, EdgeId> {
    id: NodeId,
    data: Rc<dyn INode<NodeId = NodeId, EdgeId = EdgeId>>,
}

impl<NodeId, EdgeId> Node<NodeId, EdgeId> {
    pub fn new(id: NodeId, data: Rc<dyn INode<NodeId = NodeId, EdgeId = EdgeId>>) -> Self {
        Node { id: id, data: data }
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn data(&self) -> Rc<dyn INode<NodeId = NodeId, EdgeId = EdgeId>> {
        self.data.clone()
    }
}

pub trait INode: Debug {
    type NodeId;
    type EdgeId;
    fn references(&self) -> ReferenceList<Self::NodeId, Self::EdgeId>;
}

//impl INode for () {}
