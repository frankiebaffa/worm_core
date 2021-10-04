use crate::DbCtx;
use rusqlite::{
    Error,
    Row,
};
pub trait AttachedDbType {
    fn get_name(&self) -> String;
}
pub trait DbModel<T: DbCtx, A: AttachedDbType>: Sized {
    const DB: &'static str;
    const TABLE: &'static str;
    const ALIAS: &'static str;
    fn from_row(row: &Row) -> Result<Self, Error>;
    fn get_attached_db_type() -> A;
}

