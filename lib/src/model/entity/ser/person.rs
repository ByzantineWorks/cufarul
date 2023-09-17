use super::{CompositionRepr, ModelRepr, PerformanceRepr};
use crate::model::CollectionKey;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PersonRepr {
    pub id: CollectionKey,
    pub name: String,
    pub compositions: Vec<CompositionRepr>,
    pub performances: Vec<PerformanceRepr>,
}

impl ModelRepr for PersonRepr {}
