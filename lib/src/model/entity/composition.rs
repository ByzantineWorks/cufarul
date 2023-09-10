use super::identity::PersonId;
use super::performance::Performance;
use super::property::{Property, ReferenceProperty};
use super::publication::Reference;
use super::{Model, ReferenceKey};
use crate::db::NodeLike;
use crate::model::identity::{TaxonomyId, TextId};
use crate::model::property::TranslatableProperty;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Composition {
    name: Option<TranslatableProperty>,
    text: ReferenceProperty,
    author: ReferenceProperty,
    performances: Vec<Performance>,
    publications: Vec<Reference>,
    category: ReferenceProperty,
    tags: Option<Vec<ReferenceProperty>>,
}

impl Model for Composition {}
impl NodeLike for Composition {
    type ReferenceId = ReferenceKey;

    fn references(&self) -> Vec<Self::ReferenceId> {
        let mut refs = Vec::<Self::ReferenceId>::new();

        let (_, author_id) = self.author.value(None).unwrap();
        let (_, text_id) = self.text.value(None).unwrap();
        let (_, category_id) = self.category.value(None).unwrap();
        refs.extend_from_slice(&[
            ReferenceKey::AuthoredBy(PersonId::new(author_id)),
            ReferenceKey::UsesText(TextId::new(text_id)),
            ReferenceKey::OfKind(TaxonomyId::new(category_id)),
        ]);

        for entry in &self.performances {
            refs.extend(entry.references());
        }

        for entry in &self.publications {
            refs.extend(entry.into.references());
        }

        if let Some(tags) = &self.tags {
            refs.extend(tags.iter().map(|e| {
                let (_, id) = e.value(None).unwrap();
                ReferenceKey::OfKind(TaxonomyId::new(id))
            }));
        }

        refs
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
