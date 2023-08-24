use super::{Edge, EdgeHash, Error, Identify, Node, Result, UniqueId};
use std::{
    collections::{BTreeMap, HashSet},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

pub type EdgeId = EdgeHash;
pub type NodeId = UniqueId;

type EdgeList<E> = HashSet<Rc<Edge<E>>>;

type NodeMap<N> = BTreeMap<NodeId, Node<N>>;
type EdgeMap<E> = BTreeMap<EdgeId, EdgeList<E>>;

#[derive(Debug)]
pub struct Database<E, N>
where
    E: Debug + Hash + Eq + Identify,
    N: Debug,
{
    nodes: NodeMap<N>,
    incoming: EdgeMap<E>,
    outgoing: EdgeMap<E>,
}

impl<E, N> Default for Database<E, N>
where
    E: Debug + Hash + Eq + Identify,
    N: Debug,
{
    fn default() -> Self {
        Database {
            nodes: NodeMap::default(),
            incoming: EdgeMap::default(),
            outgoing: EdgeMap::default(),
        }
    }
}

impl<E, N> Database<E, N>
where
    E: Debug + Into<String> + Hash + Eq + Identify,
    N: Debug,
{
    pub fn insert_node(&mut self, id: UniqueId, data: N) -> Option<&Node<N>> {
        Some(
            self.nodes
                .entry(id.to_owned())
                .or_insert(Node::<N>::new(id.to_owned(), data)),
        )
    }

    pub fn insert_edge(&mut self, sub: NodeId, obj: NodeId, pred: E) -> Result<()> {
        if !self.nodes.contains_key(&sub) {
            return Err(Error::InvalidUniqueId(sub.to_owned()));
        }

        if !self.nodes.contains_key(&obj) {
            return Err(Error::InvalidUniqueId(obj.to_owned()));
        }

        let edge = Rc::new(Edge::new(sub.to_owned(), obj.to_owned(), pred));
        self.outgoing
            .entry(EdgeHash::outgoing(&edge))
            .or_insert_with(EdgeList::<E>::new)
            .insert(edge.clone());

        self.incoming
            .entry(EdgeHash::incoming(&edge))
            .or_insert_with(EdgeList::<E>::new)
            .insert(edge.clone());

        Ok(())
    }
}
