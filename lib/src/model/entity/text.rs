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
pub struct TextVariation {
    pub name: TranslatableProperty,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextMembership {
    pub text: ReferenceProperty,
    pub variations: Option<Vec<NonEmptyString>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    pub name: TranslatableProperty,
    pub author: Option<ReferenceProperty>,
    pub part_of: Option<Vec<TextMembership>>,
    pub variations: Option<HashMap<String, TextVariation>>,
}

impl Model for Text {
    fn references(&self) -> Vec<ReferenceKey> {
        let mut references = Vec::<ReferenceKey>::new();
        if let Some(author) = &self.author {
            let author_id = author.value();
            references.push(ReferenceKey::WrittenBy(PersonId::new(author_id.key())));
        }

        if let Some(masters) = &self.part_of {
            references.extend(masters.iter().flat_map(|r| {
                match &r.variations {
                    Some(variations) => variations
                        .iter()
                        .map(|v| {
                            ReferenceKey::PartOf(
                                TextId::new(r.text.value().key()),
                                Some(v.value().to_owned()),
                            )
                        })
                        .collect(),
                    None => vec![ReferenceKey::PartOf(
                        TextId::new(r.text.value().key()),
                        None,
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
