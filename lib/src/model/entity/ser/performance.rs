use super::{LinkRepr, ModelRepr, PersonRepr, ReferenceRepr};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PerformanceRepr {
    pub performer: ReferenceRepr<PersonRepr>,
    pub link: LinkRepr,
}

impl ModelRepr for PerformanceRepr {}
