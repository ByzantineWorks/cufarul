mod collections;
mod composition;
mod identity;
mod interface;
mod person;
mod references;

pub use collections::CollectionKey;
pub use composition::Composition;
pub use identity::{CompositionId, PersonId};
pub use interface::Model;
pub use person::Person;
pub use references::ReferenceKey;

pub use interface::from_file;
