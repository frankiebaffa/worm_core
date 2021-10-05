use rusqlite::{
    Error,
    Row,
};
pub trait AttachedDbType {
    fn get_name(&self) -> String;
}
pub trait DbModel: Sized {
    const DB: &'static str;
    const TABLE: &'static str;
    const ALIAS: &'static str;
    fn from_row(row: &Row) -> Result<Self, Error>;
}

