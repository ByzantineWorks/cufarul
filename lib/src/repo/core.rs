use super::{error::Result, index::Index};
use crate::db::Database;

pub trait Repository {
    type DbType: Database;

    fn db_mut(&mut self) -> &mut Self::DbType;
    fn db(&self) -> &Self::DbType;
    fn index(&self) -> Result<Index>;

    // TODO: what to do with the error?
    fn sync(&mut self) -> crate::error::Result<()>;
}
