use super::{
    edge::{Edge, EdgeId, EdgeRef, ReferenceIdentity},
    error::Result,
    node::{Node, NodeRef},
    NodeIdentity,
};
use std::slice::Iter;

/// The public interface of the database.
pub trait Database
where
    Self: Default,
{
    /// The type used to identify nodes. It is required to implement the
    /// `NodeIdentity` trait. A value of this type shall uniquely identify one
    /// and only one node in the database.
    type NodeId: NodeIdentity;

    /// The type used to identify references to nodes. Note that this is not
    /// a fully specified edge, but only a reference to another node. It is
    /// required to implement the `ReferenceIdentity` trait with the generic
    /// parameter `NodeId` since the references must point to nodes within the
    /// database.
    type ReferenceId: ReferenceIdentity<Self::NodeId>;

    /// The iterator type used to iterate through nodes. It is requirede to
    /// implement the `Iterator` trait and to yield pairs of `NodeId` and
    /// and the data associated with the respective node.
    type NodeIter: Iterator<Item = (Self::NodeId, NodeRef<Self::ReferenceId>)>;

    /// Inserts a new node into the database with the given `id` and
    /// associated data `node`. Returns the same `NodeId` im case of success,
    /// and an error value otherwise.
    fn insert_node(
        &mut self,
        id: Self::NodeId,
        node: NodeRef<Self::ReferenceId>,
    ) -> Result<Node<Self::NodeId, Self::ReferenceId>>;

    /// Inserts a new edge into the database with the given fully specified
    /// edge identification `id` and associated data `edge`. Returns the same
    /// `EdgeId` in case of success, and an error value otherwise.
    fn insert_edge(
        &mut self,
        id: EdgeId<Self::NodeId, Self::ReferenceId>,
        edge: EdgeRef,
    ) -> Result<Edge<Self::NodeId, Self::ReferenceId>>;

    /// Returns an iterator over all the nodes.
    fn nodes_iter(&self) -> Self::NodeIter;

    /// Returns a boolean value according to whether there is a node associated
    /// with the given `id` or not.
    fn has_node(&self, id: Self::NodeId) -> bool;

    /// Returns a boolean value according to whether there is an edge associated
    /// with the given `id` or not.
    fn has_edge(&self, id: EdgeId<Self::NodeId, Self::ReferenceId>) -> bool;

    /// Returns the node with the given `id`, or `None` if no such node is
    /// found. The returned type is a wrapper over `NodeId` and the associated
    /// data.
    fn node_by_id(&self, id: Self::NodeId) -> Option<Node<Self::NodeId, Self::ReferenceId>>;

    /// Returns the edge with the given `id`, or `None` if no such edge is
    /// found. The returned type is a wrapper over `EdgeId` and the associated
    /// data.
    fn edge_by_id(
        &self,
        id: EdgeId<Self::NodeId, Self::ReferenceId>,
    ) -> Option<Edge<Self::NodeId, Self::ReferenceId>>;

    /// Executes complex queries on the database and returns the result.
    fn query(&self) -> Iter<Node<Self::NodeId, Self::ReferenceId>>;
}
