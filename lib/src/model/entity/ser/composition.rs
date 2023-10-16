use crate::model::CollectionKey;

use super::{
    ModelRepr, MusicalRepr, PerformanceRepr, PersonRepr, ReferenceInPublucationRepr, ReferenceRepr,
    TaxonomyRepr, TextReferenceRepr,
};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct CompositionRepr {
    pub id: CollectionKey,
    pub name: String,
    pub author: ReferenceRepr<PersonRepr>,
    pub text: TextReferenceRepr,
    pub performances: Vec<PerformanceRepr>,
    pub publications: Vec<ReferenceInPublucationRepr>,
    pub category: TaxonomyRepr,
    pub tags: Vec<TaxonomyRepr>,
    pub contribution: ContributionRepr,
    pub musical: MusicalRepr,
}

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContributionRepr {
    Composition,
    Translation { name: String, id: CollectionKey },
    Modification { name: String, id: CollectionKey },
    Unknown,
}

impl ModelRepr for CompositionRepr {}
impl ModelRepr for ContributionRepr {}
