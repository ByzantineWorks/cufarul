use super::{CompositionRepr, ModelRepr, ReferenceRepr};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ModeRepr {
    pub name: String,
    pub index: u8,
    pub compositions: Vec<ReferenceRepr<CompositionRepr>>,
}

#[derive(Clone, Serialize)]
pub struct MusicalRepr {
    pub mode: String,
    pub index: u8,
    pub base: String,
    pub modulations: Vec<String>,
}

impl ModelRepr for ModeRepr {}
impl ModelRepr for MusicalRepr {}
