use std::any::Any;

use super::Model;
use crate::{
    db::{INode, ReferenceSpec},
    model::{CollectionKey, ReferenceKey},
    serde::{GenericProperty, NonEmptyString, Property, ReferenceProperty, TranslatableProperty},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    something: GenericProperty<NonEmptyString>,
    name: TranslatableProperty,
    author: ReferenceProperty,
}

impl INode for Person {
    type NodeId = CollectionKey;
    type EdgeId = ReferenceKey;

    fn references(&self) -> crate::db::ReferenceList<Self::NodeId, Self::EdgeId> {
        let (collection, id) = self.author.value(None).unwrap();
        /* Todo: proper error reporting in case of illegal reference */
        assert!(collection.unwrap_or("people".to_owned()).eq("people"));
        vec![ReferenceSpec::new(
            CollectionKey::Person(id),
            ReferenceKey::Author,
        )]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Model for Person {}
