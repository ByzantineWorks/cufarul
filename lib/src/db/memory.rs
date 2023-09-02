use super::{
    core::Database,
    edge::{Edge, EdgeId, EdgeRef, ReferenceIdentity},
    error::{Error, Result},
    node::{Node, NodeIdentity, NodeRef},
};
use std::collections::HashMap;

type NodeMap<N, R> = HashMap<N, NodeRef<R>>;
type EdgeMap<N, R> = HashMap<R, HashMap<N, EdgeRef>>;

/// In-memory database implementation.
///
/// TODO: remove Debug
#[derive(Debug)]
pub struct Datastore<N, R> {
    nodes: NodeMap<N, R>,
    edges: EdgeMap<N, R>,
}

impl<N, R> Database for Datastore<N, R>
where
    N: NodeIdentity + 'static,
    R: ReferenceIdentity<N>,
{
    type NodeId = N;
    type ReferenceId = R;
    type NodeIter = std::collections::hash_map::IntoIter<Self::NodeId, NodeRef<Self::ReferenceId>>;

    fn insert_node(
        &mut self,
        id: Self::NodeId,
        node: NodeRef<Self::ReferenceId>,
    ) -> Result<Node<Self::NodeId, Self::ReferenceId>> {
        self.nodes.insert(id.to_owned(), node.clone()).map_or_else(
            || Ok(Node::new(id.to_owned(), node)),
            |_| Err(Error::Exists(id.to_string())),
        )
    }

    fn insert_edge(
        &mut self,
        id: EdgeId<Self::NodeId, Self::ReferenceId>,
        edge: EdgeRef,
    ) -> Result<Edge<Self::NodeId, Self::ReferenceId>> {
        if !self.nodes.contains_key(id.subject()) {
            return Err(Error::InvalidEdge(id.to_string(), id.subject().to_string()));
        }

        if !self.nodes.contains_key(&id.predicate().object()) {
            return Err(Error::InvalidEdge(
                id.to_string(),
                id.predicate().object().to_string(),
            ));
        }

        self.edges
            .entry(id.predicate().to_owned())
            .or_default()
            .insert(id.subject().to_owned(), edge.clone())
            .map_or_else(
                || Ok(Edge::new(id.to_owned(), edge.clone())),
                |_| Err(Error::Exists(id.to_string())),
            )
    }

    fn nodes_iter(&self) -> Self::NodeIter {
        self.nodes.clone().into_iter()
    }

    fn has_node(&self, id: Self::NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    fn has_edge(&self, _id: EdgeId<Self::NodeId, Self::ReferenceId>) -> bool {
        unimplemented!()
    }

    fn node_by_id(&self, id: Self::NodeId) -> Option<Node<Self::NodeId, Self::ReferenceId>> {
        self.nodes.get(&id).map(|elem| Node::new(id, elem.clone()))
    }

    fn edge_by_id(
        &self,
        id: EdgeId<Self::NodeId, Self::ReferenceId>,
    ) -> Option<Edge<Self::NodeId, Self::ReferenceId>> {
        self.edges
            .get(&id.predicate())
            .and_then(|refs| refs.get(&id.subject()))
            .map(|elem| Edge::new(id, elem.clone()))
    }

    fn query(&self) -> std::slice::Iter<Node<Self::NodeId, Self::ReferenceId>> {
        unimplemented!()
    }
}

impl<N, R> Default for Datastore<N, R> {
    fn default() -> Self {
        Datastore {
            nodes: NodeMap::default(),
            edges: EdgeMap::default(),
        }
    }
}
