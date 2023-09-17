use super::ModelRepr;
use crate::model::CollectionKey;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct TaxonomyRepr {
    pub id: CollectionKey,
    pub name: String,
    pub parents: Vec<(String, CollectionKey)>,
}

impl ModelRepr for TaxonomyRepr {}
