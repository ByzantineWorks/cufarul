use super::identity::PersonId;
use super::performance::Performance;
use super::property::ReferenceProperty;
use super::publication::Reference;
use super::{Model, ReferenceKey};
use crate::db::NodeLike;
use crate::model::identity::{PublicationId, TaxonomyId, TextId};
use crate::model::property::{ContributionProperty, ModeProperty, TranslatableProperty};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Composition {
    pub name: TranslatableProperty,
    pub text: ReferenceProperty,
    pub author: ReferenceProperty,
    pub performances: Vec<Performance>,
    pub publications: Vec<Reference>,
    pub category: ReferenceProperty,
    pub tags: Option<Vec<ReferenceProperty>>,
    pub contribution: Option<ContributionProperty>,
    pub musical: ModeProperty,
}

impl Model for Composition {
    fn references(&self) -> Vec<ReferenceKey> {
        let mut refs = Vec::<ReferenceKey>::new();

        let author_id = self.author.value();
        let text_id = self.text.value();
        let category_id = self.category.value();
        refs.extend_from_slice(&[
            ReferenceKey::AuthoredBy(PersonId::new(author_id.key())),
            ReferenceKey::UsesText(TextId::new(text_id.key())),
            ReferenceKey::OfKind(TaxonomyId::new(category_id.key())),
        ]);

        for entry in &self.performances {
            refs.extend(entry.references());
        }

        refs.extend(self.publications.iter().map(|e| {
            let publication_id = e.into.value();
            ReferenceKey::PublishedBy(PublicationId::new(publication_id.key()))
        }));

        if let Some(tags) = &self.tags {
            refs.extend(tags.iter().map(|e| {
                let id = e.value();
                ReferenceKey::OfKind(TaxonomyId::new(id.key()))
            }));
        }

        refs
    }
}
impl NodeLike for Composition {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
