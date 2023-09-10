use crate::{
    db::NodeIdentity,
    repo::{Error, Result},
};

macro_rules! count {
    () => (0usize);
    ($x:literal $($xs:literal)*) => (1usize + count!($($xs)*));
}

macro_rules! EntityId {
    ($name:ident) => {
        #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(String);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.0.as_str())
            }
        }

        impl $name {
            pub fn new(id: String) -> Self {
                $name(id)
            }
        }
    };
}

#[macro_export]
macro_rules! EntitiyKey {
    ($name:ident, $(($entry:ident, $type:ident, $id:literal)), +) => {
        pub mod types {
            $(
                EntityId!($type);
            )+
        }

        #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $(
                $entry(self::types::$type),
            )+
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let (collection, id) = match self {
                    $(
                        Self::$entry(id) => ($id, id.to_string()),
                    )+
                };

                f.write_fmt(format_args!("@{}/{}", collection, id))
            }
        }

        impl $name {
            pub fn into_iter() -> std::array::IntoIter<&'static str, {count!($($id)*)}> {
                const COLLECTIONS: [&str; count!($($id)*)] = [$($id, )+];
                COLLECTIONS.into_iter()
            }

            pub fn from_collection_and_id(collection: String, id: String) -> Result<Self> {
                match collection.as_str() {
                    $(
                        $id => Ok(Self::$entry(self::types::$type::new(id))),
                    )+
                    _ => Err(Error::InvalidCollectionKey(collection)),
                }
            }
        }
    };
}

EntitiyKey!(
    CollectionKey,
    (Person, PersonId, "people"),
    (Composition, CompositionId, "compositions")
);

// TODO: use smarter macros!

// #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub enum CollectionKey {
//     Person(PersonId),
//     Composition(CompositionId),
// }

// impl Display for CollectionKey {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let (collection, id) = match self {
//             Self::Person(id) => ("people", id.to_string()),
//             Self::Composition(id) => ("compositions", id.to_string()),
//         };

//         f.write_fmt(format_args!("@{}/{}", collection, id))
//     }
// }

// impl CollectionKey {
//     pub fn into_iter() -> std::array::IntoIter<&'static str, 2> {
//         const COLLECTIONS: [&str; 2] = ["people", "compositions"];
//         COLLECTIONS.into_iter()
//     }

//     pub fn from_collection_and_id(collection: String, id: String) -> Result<Self> {
//         match collection.as_str() {
//             "people" => Ok(CollectionKey::Person(PersonId::new(id))),
//             "compositions" => Ok(CollectionKey::Composition(CompositionId::new(id))),
//             _ => Err(Error::InvalidCollectionKey(collection)),
//         }
//     }
// }

impl NodeIdentity for CollectionKey {}
