use super::{CompositionRepr, ModelRepr, PersonRepr, ReferenceRepr};
use crate::model::CollectionKey;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct TextRepr {
    pub id: CollectionKey,
    pub name: String,
    pub author: Option<ReferenceRepr<PersonRepr>>,
    pub compositions: Vec<ReferenceRepr<CompositionRepr>>,
}

impl ModelRepr for TextRepr {}
