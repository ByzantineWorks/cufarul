use super::{CompositionRepr, ModelRepr, PersonRepr, ReferenceRepr};
use crate::model::CollectionKey;
use serde::Serialize;
use std::{collections::HashMap, fmt::Display, vec};

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde[untagged, into = "String"]]
pub enum TextVariantType {
    Main,
    Variant(String),
}

impl Default for TextVariantType {
    fn default() -> Self {
        Self::Main
    }
}

impl From<Option<String>> for TextVariantType {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => Self::Variant(v),
            None => Self::Main,
        }
    }
}

impl From<String> for TextVariantType {
    fn from(value: String) -> Self {
        Self::Variant(value)
    }
}

impl From<TextVariantType> for String {
    fn from(value: TextVariantType) -> Self {
        value.to_string()
    }
}

impl Display for TextVariantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Main => f.write_str("~~main~~"),
            Self::Variant(v) => f.write_str(v.as_str()),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct TextVariantMap(HashMap<TextVariantType, TextVariantRepr>);

#[derive(Clone, Serialize)]
pub struct TextVariantRepr {
    pub name: String,
    pub subtexts: Vec<ReferenceRepr<TextRepr>>,
    pub compositions: Vec<ReferenceRepr<CompositionRepr>>,
}

#[derive(Clone, Serialize)]
pub struct TextRepr {
    pub id: CollectionKey,
    pub author: Option<ReferenceRepr<PersonRepr>>,
    pub variants: TextVariantMap,
}

impl ModelRepr for TextRepr {}
impl TextRepr {
    pub fn has_compositions(&self, variant: Option<TextVariantType>) -> bool {
        match variant {
            Some(variant) => self
                .variants
                .0
                .get(&variant)
                .map_or(false, |t| t.compositions.len() > 0),
            None => self.variants.0.values().all(|t| t.compositions.len() > 0),
        }
    }
}

impl TextVariantMap {
    pub fn new(main_variant: String) -> Self {
        TextVariantMap(HashMap::from([(
            TextVariantType::Main,
            TextVariantRepr {
                name: main_variant,
                subtexts: vec![],
                compositions: vec![],
            },
        )]))
    }

    pub fn main_variant(&self) -> &TextVariantRepr {
        self.0.get(&TextVariantType::Main).unwrap()
    }

    pub fn variants(&self) -> impl Iterator<Item = (&TextVariantType, &TextVariantRepr)> {
        self.0.iter()
    }

    pub fn push_variant(&mut self, variant: TextVariantType, name: String) {
        self.0.insert(
            variant,
            TextVariantRepr {
                name: name,
                subtexts: vec![],
                compositions: vec![],
            },
        );
    }

    pub fn push_variant_subtext(
        &mut self,
        variant: Option<TextVariantType>,
        data: ReferenceRepr<TextRepr>,
    ) {
        self.0.entry(variant.unwrap_or_default()).and_modify(|v| {
            v.subtexts.push(data);
        });
    }

    pub fn push_variant_composition(
        &mut self,
        variant: Option<TextVariantType>,
        data: ReferenceRepr<CompositionRepr>,
    ) {
        self.0.entry(variant.unwrap_or_default()).and_modify(|v| {
            v.compositions.push(data);
        });
    }

    pub fn has_variant(&self, variant: Option<TextVariantType>) -> bool {
        self.0.contains_key(&variant.unwrap_or_default())
    }
}
