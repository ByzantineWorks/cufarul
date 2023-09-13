//! # `cufarul::db`
//!
//! `cufarul::db` implements the in-memory graph database which stores the data
//! in the Byzantine Music Database.
//!
//! The database defines nodes and edges. Nodes are documents with properties
//! which represent composers, compositions, texts and so on. Nodes are
//! organized in collections. Each node is part of one and only one collection.
//! Edges are directed and connect nodes.
//!
//! The database implementation does not care about the content of the documents
//! or the semantics of the edges. For this reason, no semantic validation is
//! applied at the level of the database interface. The implementation only
//! cares to validate if the nodes and edges form a consistent directed graph.
//!
//! The database interfaces provides methods to:
//!
//! - insert a node into the database with a given key
//! - insert an edge into the database with a given key
//! - retrieve a node based on the node key
//! - retrieve an edge based on the edge key
//! - query the graph based on generic specifications
//!
//! The entry point to the database is through the `Datastore` struct.

mod core;
mod edge;
mod error;
mod memory;
mod node;

pub use self::core::Database;
pub use self::edge::{Edge, EdgeId, EdgeLike, EdgeRef, ReferenceIdentity};
pub use self::error::{Error, Result};
pub use self::memory::Datastore;
pub use self::node::{Node, NodeIdentity, NodeLike, NodeRef};
