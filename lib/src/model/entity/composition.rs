use super::identity::PersonId;
use super::performance::Performance;
use super::property::ReferenceProperty;
use super::publication::Reference;
use super::{Model, ReferenceKey};
use crate::db::NodeLike;
use crate::model::identity::{PublicationId, TaxonomyId, TextId};
use crate::model::property::{ContributionProperty, TranslatableProperty};
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
    contribution: Option<ContributionProperty>,
}

impl Model for Composition {}
impl NodeLike for Composition {
    type ReferenceId = ReferenceKey;

    fn references(&self) -> Vec<Self::ReferenceId> {
        let mut refs = Vec::<Self::ReferenceId>::new();

        let author_id = self.author.value();
        let text_id = self.text.value();
        let category_id = self.category.value();
        refs.extend_from_slice(&[
            ReferenceKey::AuthoredBy(PersonId::new(author_id.id())),
            ReferenceKey::UsesText(TextId::new(text_id.id())),
            ReferenceKey::OfKind(TaxonomyId::new(category_id.id())),
        ]);

        for entry in &self.performances {
            refs.extend(entry.references());
        }

        refs.extend(self.publications.iter().map(|e| {
            let publication_id = e.into.value();
            ReferenceKey::PublishedBy(PublicationId::new(publication_id.id()))
        }));

        if let Some(tags) = &self.tags {
            refs.extend(tags.iter().map(|e| {
                let id = e.value();
                ReferenceKey::OfKind(TaxonomyId::new(id.id()))
            }));
        }

        refs
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
