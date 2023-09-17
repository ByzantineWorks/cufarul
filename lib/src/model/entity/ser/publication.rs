use super::{CompositionRepr, ModelRepr, PersonRepr, ReferenceRepr};
use crate::model::CollectionKey;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PublicationRepr {
    pub id: CollectionKey,
    pub name: String,
    pub author: ReferenceRepr<PersonRepr>,
    pub compositions: Vec<ReferenceRepr<CompositionRepr>>,
}

#[derive(Clone, Serialize)]
pub struct ReferenceInPublucationRepr {
    pub into: ReferenceRepr<PublicationRepr>,
    pub page: u16,
}

impl ModelRepr for PublicationRepr {}
impl ModelRepr for ReferenceInPublucationRepr {}
