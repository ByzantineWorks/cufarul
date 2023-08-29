pub struct ReferenceSpec<NodeId, EdgeId> {
    pub target: NodeId,
    pub predicate: EdgeId,
}

impl<NodeId, EdgeId> ReferenceSpec<NodeId, EdgeId> {
    pub fn new(target: NodeId, predicate: EdgeId) -> Self {
        ReferenceSpec {
            target: target,
            predicate: predicate,
        }
    }
}

pub type ReferenceList<NodeId, EdgeId> = Vec<ReferenceSpec<NodeId, EdgeId>>;
