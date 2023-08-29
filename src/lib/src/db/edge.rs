use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct Edge<NodeId, EdgeId> {
    subject: NodeId,
    object: NodeId,
    predicate: EdgeId,
    data: Option<Rc<dyn IEdge>>,
}

impl<NodeId, EdgeId> Edge<NodeId, EdgeId> {
    pub fn new(subject: NodeId, object: NodeId, pred: EdgeId, data: Option<Rc<dyn IEdge>>) -> Self {
        Edge {
            subject: subject,
            object: object,
            predicate: pred,
            data: data,
        }
    }

    pub fn subject(&self) -> &NodeId {
        &self.subject
    }

    pub fn object(&self) -> &NodeId {
        &self.object
    }

    pub fn predicate(&self) -> &EdgeId {
        &self.predicate
    }

    pub fn data(&self) -> Option<Rc<dyn IEdge>> {
        self.data.clone()
    }
}

pub trait IEdge: Debug {}
