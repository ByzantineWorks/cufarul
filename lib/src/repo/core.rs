use super::{error::Result, index::RepoIndex};
use crate::db::Database;

pub trait Repository {
    type DbType: Database;

    fn db_mut(&mut self) -> &mut Self::DbType;
    fn db(&self) -> &Self::DbType;
    fn index(&self) -> Result<RepoIndex>;

    // TODO: what to do with the error?
    fn sync(&mut self) -> crate::error::Result<()>;
}
