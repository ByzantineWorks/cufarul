mod collections;
mod references;

// use crate::db::NodeIdentity;

// macro_rules! EntityId {
//     ($name:ident) => {
//         #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
//         pub struct $name(String);

//         impl Display for $name {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 f.write_str(self.0.as_str())
//             }
//         }

//         impl $name {
//             pub fn new(id: String) -> Self {
//                 $name(id)
//             }
//         }
//     };
// }

// EntityId!(PersonId);
// EntityId!(CompositionId);

pub use self::collections::types::*;
pub use self::collections::CollectionKey;
pub use self::references::ReferenceKey;
