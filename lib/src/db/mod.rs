mod database;
mod edge;
mod error;
mod node;
mod uid;

use std::fmt::Debug;
use std::hash::Hash;

pub use database::Database;
pub use edge::Edge;
pub use edge::IEdge;
pub use error::{Error, Result};
pub use node::INode;
pub use node::Node;
pub use uid::ReferenceId;

pub trait Identity:
    Clone + Debug + Hash + Ord + PartialOrd + Eq + PartialEq + Into<String>
{
}
