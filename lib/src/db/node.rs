use std::{fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct Node<NodeId> {
    id: NodeId,
    data: Rc<dyn INode>,
}

impl<NodeId> Node<NodeId> {
    pub fn new(id: NodeId, data: Rc<dyn INode>) -> Self {
        Node { id: id, data: data }
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn data(&self) -> Rc<dyn INode> {
        self.data.clone()
    }
}

pub trait INode: Debug {}
