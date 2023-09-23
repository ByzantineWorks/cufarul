use std::collections::HashMap;

use super::{Model, Query};
use crate::{
    db::NodeLike,
    model::{
        identity::{PersonId, TextId},
        property::{ReferenceProperty, TranslatableProperty},
        serde::NonEmptyString,
        ReferenceKey,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextVariant {
    pub name: TranslatableProperty,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextMembership {
    pub text: ReferenceProperty,
    pub variants: Option<Vec<NonEmptyString>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    pub name: TranslatableProperty,
    pub author: Option<ReferenceProperty>,
    pub part_of: Option<Vec<TextMembership>>,
    pub variants: Option<HashMap<String, TextVariant>>,
}

impl Model for Text {
    fn references(&self) -> Vec<ReferenceKey> {
        let mut references = Vec::<ReferenceKey>::new();
        if let Some(author) = &self.author {
            let author_id = author.value();
            references.push(ReferenceKey::WrittenBy(
                PersonId::new(author_id.key()),
                author.variant().into(),
            ));
        }

        if let Some(masters) = &self.part_of {
            references.extend(masters.iter().flat_map(|r| {
                match &r.variants {
                    Some(variations) => variations
                        .iter()
                        .map(|v| {
                            ReferenceKey::PartOf(
                                TextId::new(r.text.value().key()),
                                crate::model::TextVariantType::Variant(v.value().to_owned()),
                            )
                        })
                        .collect(),
                    None => vec![ReferenceKey::PartOf(
                        TextId::new(r.text.value().key()),
                        crate::model::TextVariantType::Main,
                    )],
                }
            }));
        }

        references
    }
}

impl NodeLike for Text {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Query for Text {
    fn contains(&self, value: String) -> bool {
        self.name.contains(value)
    }
}
