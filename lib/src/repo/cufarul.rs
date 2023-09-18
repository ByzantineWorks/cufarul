use super::{
    core::{Cufarul, Repository},
    error::{Error, Result},
    index::{LoadPath, RepoIndex},
    spec::RepositorySpec,
    REPOSITORY_SUPPORTED_VERSION,
};
use crate::{
    db::{Database, Datastore, EdgeId, ReferenceIdentity},
    model::{
        AsBoxModelRepr, CollectionKey, Composition, CompositionRepr, Contribution,
        ContributionRepr, Lang, LinkRepr, ModeRepr, ModelRepr, ModelReprRef, MusicalRepr,
        Performance, PerformanceRepr, Person, PersonRepr, Publication, PublicationRepr,
        ReferenceInPublucationRepr, ReferenceKey, ReferenceRepr, Taxonomy, TaxonomyRepr, Text,
        TextRepr,
    },
};
use std::{collections::HashMap, sync::Arc};

type ReferenceList = Vec<EdgeId<CollectionKey, ReferenceKey>>;

pub struct CufarulRepository {
    spec: RepositorySpec,
    db: Datastore<CollectionKey, ReferenceKey>,
}

impl Repository for CufarulRepository {
    type DbType = Datastore<CollectionKey, ReferenceKey>;

    fn db(&self) -> &Self::DbType {
        &self.db
    }

    fn db_mut(&mut self) -> &mut Self::DbType {
        &mut self.db
    }

    fn index(&self) -> Result<RepoIndex> {
        let mut index = RepoIndex::new();

        for collection in CollectionKey::into_iter() {
            let base = self.spec.root().join(collection);
            let mut fragment = base
                .is_dir()
                .then_some(std::fs::read_dir(base)?)
                .ok_or(Error::MissingCollection(collection.to_owned()))
                .map(|dir| {
                    dir.filter_map(|elem| match elem {
                        Ok(entry) => Some(entry.path()),
                        Err(_) => None,
                    })
                    .filter_map(|entry| {
                        match (
                            entry.is_file(),
                            entry.extension().unwrap_or_default().to_str(),
                        ) {
                            (true, Some("toml")) => Some(LoadPath::from(entry)),
                            _ => None,
                        }
                    })
                    .collect::<RepoIndex>()
                })?;

            index.append(&mut fragment);
        }

        Ok(index)
    }

    fn sync(&mut self) -> crate::error::Result<()> {
        let mut references = ReferenceList::new();
        for entry in self.index()? {
            let path = entry.path().unwrap_or(
                self.spec
                    .root()
                    .join(entry.collection())
                    .join(entry.id())
                    .with_extension("toml"),
            );

            let key = CollectionKey::from_collection_and_id(
                entry.collection().to_owned(),
                entry.id().to_owned(),
            )?;

            println!("Loading: {key}");

            let (node, model) = match &key {
                CollectionKey::Person(_) => {
                    crate::model::into_traits(crate::model::from_file::<Person>(path)?)
                }
                CollectionKey::Composition(_) => {
                    crate::model::into_traits(crate::model::from_file::<Composition>(path)?)
                }
                CollectionKey::Performance(_) => {
                    crate::model::into_traits(crate::model::from_file::<Performance>(path)?)
                }
                CollectionKey::Publication(_) => {
                    crate::model::into_traits(crate::model::from_file::<Publication>(path)?)
                }
                CollectionKey::Text(_) => {
                    crate::model::into_traits(crate::model::from_file::<Text>(path)?)
                }
                CollectionKey::Taxonomy(_) => {
                    crate::model::into_traits(crate::model::from_file::<Taxonomy>(path)?)
                }
            };

            self.db.insert_node(key.to_owned(), node)?;
            let mut fragment: ReferenceList = model
                .references()
                .iter()
                .map(|reference| EdgeId::new(key.to_owned(), reference.to_owned()))
                .collect();
            references.append(&mut fragment);
        }

        // TODO: check if references are circular

        for edge in references {
            // Check if object exists only, for the subject we are sure since
            // the edge came from it.
            if !self.db.has_node(edge.forward_predicate().object()) {
                return Err(Error::InvalidReference(edge.to_string()).into());
            }

            self.db.insert_edge(edge, Arc::new(()))?;
        }

        Ok(())
    }
}

impl CufarulRepository {
    pub fn from_spec(spec: RepositorySpec) -> Result<Self> {
        spec.version()
            .eq(&REPOSITORY_SUPPORTED_VERSION)
            .then_some(CufarulRepository {
                spec: spec.to_owned(),
                db: Datastore::default(),
            })
            .ok_or(Error::UnsupportedVersion(spec.version()))
    }

    pub fn spec(&self) -> &RepositorySpec {
        &self.spec
    }

    fn resolve_person(
        &self,
        id: CollectionKey,
        lang: &Option<Lang>,
        expand: bool,
    ) -> Result<PersonRepr> {
        let node = self
            .db
            .node_by_id(id.to_owned())
            .ok_or(Error::NoData(id.to_string()))?;

        let data = node
            .data()
            .as_any_arc()
            .downcast::<Person>()
            .map_err(|_| Error::InternalError("incompatible"))?;

        let mut compositions = Vec::<CompositionRepr>::new();
        // let mut performances = Vec::<PerformanceRepr>::new();

        if expand {
            for reference in node.references() {
                match reference {
                    ReferenceKey::Authored(id) => {
                        compositions.push(self.resolve_composition(
                            CollectionKey::Composition(id.to_owned()),
                            lang,
                            false,
                        )?);
                    }
                    _ => {}
                }
            }
        }

        Ok(PersonRepr {
            id: id,
            name: data.name.value(lang.to_owned()),
            compositions: compositions,
            performances: vec![],
        })
    }

    fn resolve_text(
        &self,
        id: CollectionKey,
        lang: &Option<Lang>,
        expand: bool,
    ) -> Result<TextRepr> {
        let node = self
            .db
            .node_by_id(id.to_owned())
            .ok_or(Error::NoData(id.to_string()))?;
        let data = node
            .data()
            .as_any_arc()
            .downcast::<Text>()
            .map_err(|_| Error::InternalError("incompatible"))?;

        let author = match (expand, data.author.clone()) {
            (true, Some(reference)) => Some(ReferenceRepr::Model(self.resolve_person(
                reference.value(),
                &lang,
                false,
            )?)),
            (false, Some(reference)) => Some(ReferenceRepr::Key(reference.value())),
            (_, None) => None,
        };

        let compositions: Vec<ReferenceRepr<CompositionRepr>> = node
            .references()
            .filter_map(|reference| match reference {
                ReferenceKey::UsedIn(id) => match expand {
                    true => self
                        .resolve_composition(CollectionKey::Composition(id.to_owned()), lang, false)
                        .map(|repr| ReferenceRepr::Model(repr))
                        .ok(),
                    false => Some(ReferenceRepr::Key(CollectionKey::Composition(
                        id.to_owned(),
                    ))),
                },
                _ => None,
            })
            .collect();

        Ok(TextRepr {
            id: id,
            name: data.name.value(lang.to_owned()),
            author: author,
            compositions: compositions,
        })
    }

    fn resolve_publication(
        &self,
        id: CollectionKey,
        lang: &Option<Lang>,
        expand: bool,
    ) -> Result<PublicationRepr> {
        let node = self
            .db
            .node_by_id(id.to_owned())
            .ok_or(Error::NoData(id.to_string()))?;
        let data = node
            .data()
            .as_any_arc()
            .downcast::<Publication>()
            .map_err(|_| Error::InternalError("incompatible"))?;

        let author = match (&data.author, expand) {
            (Some(key), true) => {
                ReferenceRepr::Model(self.resolve_person(key.value(), lang, false)?)
            }
            (Some(key), false) => ReferenceRepr::Key(key.value()),
            (None, _) => ReferenceRepr::Unavailable,
        };

        let compositions: Vec<ReferenceRepr<CompositionRepr>> = node
            .references()
            .filter_map(|reference| match reference {
                ReferenceKey::Published(id) => match expand {
                    true => self
                        .resolve_composition(CollectionKey::Composition(id.to_owned()), lang, false)
                        .map(|repr| ReferenceRepr::Model(repr))
                        .ok(),
                    false => Some(ReferenceRepr::Key(CollectionKey::Composition(
                        id.to_owned(),
                    ))),
                },
                _ => None,
            })
            .collect();

        Ok(PublicationRepr {
            id: id,
            name: data.name.value(lang.to_owned()),
            author: author,
            compositions: compositions,
        })
    }

    fn resolve_taxonomy(&self, id: CollectionKey, lang: &Option<Lang>) -> Result<TaxonomyRepr> {
        let data = self
            .db
            .node_by_id(id.to_owned())
            .ok_or(Error::NoData(id.to_string()))?
            .data()
            .as_any_arc()
            .downcast::<Taxonomy>()
            .map_err(|_| Error::InternalError("incompatible"))?;

        let mut parents: Vec<(String, CollectionKey)> = Vec::new();
        let mut current = data.clone();
        while let Some(parent_id) = current.parent.to_owned() {
            let parent = self
                .db
                .node_by_id(parent_id.value())
                .ok_or(Error::NoData(id.to_string()))?
                .data()
                .as_any_arc()
                .downcast::<Taxonomy>()
                .map_err(|_| Error::InternalError("incompatible"))?;
            current = parent.clone();

            parents.push((current.name.value(lang.to_owned()), parent_id.value()));
        }

        Ok(TaxonomyRepr {
            id: id,
            name: data.name.value(lang.to_owned()),
            parents: parents,
        })
    }

    fn resolve_composition(
        &self,
        id: CollectionKey,
        lang: &Option<Lang>,
        expand: bool,
    ) -> Result<CompositionRepr> {
        let data = self
            .db
            .node_by_id(id.to_owned())
            .ok_or(Error::NoData(id.to_string()))?
            .data()
            .as_any_arc()
            .downcast::<Composition>()
            .map_err(|_| Error::InternalError("incompatible"))?;

        let author = self.resolve_person(data.author.value(), lang, false)?;
        let text = self.resolve_text(data.text.value(), lang, false)?;
        let performances = data
            .performances
            .iter()
            .filter_map(|p| match expand {
                true => self
                    .resolve_person(p.performer.value(), lang, false)
                    .and_then(|repr| {
                        Ok(PerformanceRepr {
                            performer: ReferenceRepr::Model(repr),
                            link: p.link.clone().into(),
                        })
                    })
                    .ok(),
                false => Some(PerformanceRepr {
                    performer: ReferenceRepr::Key(p.performer.value()),
                    link: LinkRepr {
                        kind: "Sds".to_owned(),
                        url: "sd".to_owned(),
                    },
                }),
            })
            .collect::<Vec<PerformanceRepr>>();
        let publications = data
            .publications
            .iter()
            .map(|p| ReferenceInPublucationRepr {
                into: match expand {
                    true => self
                        .resolve_publication(p.into.value(), lang, false)
                        .and_then(|repr| Ok(ReferenceRepr::Model(repr)))
                        .unwrap_or(ReferenceRepr::Unavailable),
                    false => ReferenceRepr::Key(p.into.value()),
                },
                page: p.page,
            })
            .collect::<Vec<ReferenceInPublucationRepr>>();

        let category = self.resolve_taxonomy(data.category.value(), lang)?;
        let contribution = match &data.contribution {
            Some(c) => match c.value() {
                Contribution::Composition => ContributionRepr::Composition,
                Contribution::Modification(id) => ContributionRepr::Modification {
                    name: self
                        .resolve_composition(
                            CollectionKey::Composition(id.to_owned()),
                            lang,
                            false,
                        )?
                        .name,
                    id: CollectionKey::Composition(id),
                },
                Contribution::Translation(id) => ContributionRepr::Translation {
                    name: self
                        .resolve_composition(
                            CollectionKey::Composition(id.to_owned()),
                            lang,
                            false,
                        )?
                        .name,
                    id: CollectionKey::Composition(id),
                },
            },
            None => ContributionRepr::Unknown,
        };

        let musical = MusicalRepr {
            base: data.musical.base.to_string(),
            mode: data.musical.mode.to_string(),
            index: data.musical.mode.to_owned().into(),
            modulations: vec![],
        };

        Ok(CompositionRepr {
            id: id,
            name: data.name.value(lang.to_owned()),
            author: author.into(),
            text: text.into(),
            performances: performances,
            publications: publications,
            category: category.into(),
            tags: vec![],
            contribution: contribution,
            musical: musical,
        })
    }
}

impl Cufarul for CufarulRepository {
    fn model_by_id(&self, id: CollectionKey, lang: Option<Lang>) -> Result<ModelReprRef> {
        let repr: Box<dyn ModelRepr> = match id {
            CollectionKey::Person(_) => self.resolve_person(id, &lang, true)?.clone_boxed(),
            CollectionKey::Composition(_) => {
                self.resolve_composition(id, &lang, true)?.clone_boxed()
            }
            CollectionKey::Text(_) => self.resolve_text(id, &lang, true)?.clone_boxed(),
            CollectionKey::Publication(_) => {
                self.resolve_publication(id, &lang, true)?.clone_boxed()
            }
            CollectionKey::Taxonomy(_) => self.resolve_taxonomy(id, &lang)?.clone_boxed(),
            _ => todo!(),
        };

        Ok(repr)
    }

    fn compositions(&self, lang: Option<Lang>) -> Vec<CompositionRepr> {
        self.db
            .node_ids()
            .filter_map(|id| match id {
                CollectionKey::Composition(_) => self.resolve_composition(id, &lang, true).ok(),
                _ => None,
            })
            .collect::<Vec<CompositionRepr>>()
    }

    fn people(&self, lang: Option<Lang>) -> Vec<PersonRepr> {
        self.db
            .node_ids()
            .filter_map(|id| match id {
                CollectionKey::Person(_) => self.resolve_person(id, &lang, true).ok(),
                _ => None,
            })
            .collect::<Vec<PersonRepr>>()
    }

    fn texts(&self, lang: Option<Lang>) -> Vec<TextRepr> {
        self.db
            .node_ids()
            .filter_map(|id| match id {
                CollectionKey::Text(_) => self.resolve_text(id, &lang, true).ok(),
                _ => None,
            })
            .collect::<Vec<TextRepr>>()
    }

    fn modes(&self) -> HashMap<u8, ModeRepr> {
        let mut modes = HashMap::<u8, ModeRepr>::with_capacity(8);

        self.compositions(None).iter().for_each(|c| {
            modes
                .entry(c.musical.index)
                .or_insert(ModeRepr {
                    name: c.musical.mode.to_owned(),
                    index: c.musical.index,
                    compositions: vec![],
                })
                .compositions
                .push(ReferenceRepr::Model(c.to_owned()))
        });

        modes
    }
}
