use rusqlite::{
    Error,
    Row,
};
pub trait DbModel: Sized {
    const TABLE: &'static str;
    const ALIAS: &'static str;
    fn from_row(row: &Row) -> Result<Self, Error>;
}
