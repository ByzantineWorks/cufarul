use std::collections::HashMap;

use super::{error::Result, index::RepoIndex};
use crate::db::Database;
use crate::model::{
    CollectionKey, CompositionRepr, Lang, ModeRepr, ModelReprRef, PersonRepr, TextRepr,
};

pub trait Repository {
    type DbType: Database;

    fn db_mut(&mut self) -> &mut Self::DbType;
    fn db(&self) -> &Self::DbType;
    fn index(&self) -> Result<RepoIndex>;

    // TODO: what to do with the error?
    fn sync(&mut self) -> crate::error::Result<()>;
}

pub trait Cufarul {
    fn model_by_id(&self, id: CollectionKey, lang: Option<Lang>) -> Result<ModelReprRef>;

    fn compositions(&self, lang: Option<Lang>) -> Vec<CompositionRepr>;
    fn people(&self, lang: Option<Lang>) -> Vec<PersonRepr>;
    fn texts(&self, lang: Option<Lang>) -> Vec<TextRepr>;
    fn modes(&self) -> HashMap<u8, ModeRepr>;
}
