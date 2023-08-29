use super::{
    identity::Allowed, node::INode, Edge, Error, IEdge, Identity, Node, ReferenceId, Result,
};
use std::{collections::BTreeMap, collections::LinkedList, rc::Rc};

type EdgeList<NodeId, EdgeId> = LinkedList<Edge<NodeId, EdgeId>>;

type NodeMap<NodeId, EdgeId> = BTreeMap<NodeId, Node<NodeId, EdgeId>>;
type EdgeMap<NodeId, EdgeId> = BTreeMap<ReferenceId<NodeId, EdgeId>, EdgeList<NodeId, EdgeId>>;

#[derive(Debug)]
pub struct Database<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity + Allowed<NodeId>,
{
    nodes: NodeMap<NodeId, EdgeId>,
    incoming: EdgeMap<NodeId, EdgeId>,
    outgoing: EdgeMap<NodeId, EdgeId>,
}

impl<NodeId, EdgeId> Default for Database<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity + Allowed<NodeId>,
{
    fn default() -> Self {
        Database {
            nodes: NodeMap::default(),
            incoming: EdgeMap::default(),
            outgoing: EdgeMap::default(),
        }
    }
}

impl<NodeId, EdgeId> Database<NodeId, EdgeId>
where
    NodeId: Identity,
    EdgeId: Identity + Allowed<NodeId>,
{
    pub fn insert_node(
        &mut self,
        id: NodeId,
        data: Rc<dyn INode<NodeId = NodeId, EdgeId = EdgeId>>,
    ) -> Result<()> {
        self.nodes
            .entry(id.to_owned())
            .or_insert(Node::new(id.to_owned(), data));

        Ok(())
    }

    pub fn insert_edge(
        &mut self,
        sub: NodeId,
        obj: NodeId,
        pred: EdgeId,
        data: Option<Rc<dyn IEdge>>,
    ) -> Result<()> {
        if !self.nodes.contains_key(&sub) {
            return Err(Error::InvalidUniqueId(sub.into()));
        }

        if !self.nodes.contains_key(&obj) {
            return Err(Error::InvalidUniqueId(obj.into()));
        }

        if !pred.is_allowed(&sub, &obj) {
            return Err(Error::IllegalReference(
                sub.to_string(),
                pred.to_string(),
                obj.to_string(),
            ));
        }

        let edge = Edge::new(
            sub.to_owned(),
            obj.to_owned(),
            pred.to_owned(),
            data.clone(),
        );

        self.outgoing
            .entry(ReferenceId::new(sub, pred.to_owned()))
            .or_insert_with(EdgeList::new)
            .push_back(edge.to_owned());

        self.incoming
            .entry(ReferenceId::new(obj, pred.to_owned()))
            .or_insert_with(EdgeList::new)
            .push_back(edge.to_owned());

        Ok(())
    }

    pub fn has_node(&self, node: NodeId) -> bool {
        self.nodes.contains_key(&node)
    }

    pub fn get_node(&self, node: NodeId) -> Option<&Node<NodeId, EdgeId>> {
        self.nodes.get(&node)
    }

    pub fn nodes(&self) -> std::collections::btree_map::Iter<NodeId, Node<NodeId, EdgeId>> {
        self.nodes.iter()
    }

    pub fn edges_from(
        &self,
        target: NodeId,
        predicate: EdgeId,
    ) -> Option<std::collections::linked_list::Iter<Edge<NodeId, EdgeId>>> {
        let reference = ReferenceId::new(target, predicate);
        self.outgoing
            .get(&reference)
            .and_then(|entry| Some(entry.iter()))
    }

    pub fn edges_to(
        &self,
        target: NodeId,
        predicate: EdgeId,
    ) -> Option<std::collections::linked_list::Iter<Edge<NodeId, EdgeId>>> {
        let reference = ReferenceId::new(target, predicate);
        self.incoming
            .get(&reference)
            .and_then(|entry| Some(entry.iter()))
    }
}
