use rusqlite::{
    Error,
    Row,
    types::FromSql,
};
pub trait ColumnValue {
    fn value<'a, T: FromSql>(&self, column: &'a str) -> Result<T, Error>;
}
impl<'stmt> ColumnValue for Row<'stmt> {
    fn value<'a, T: FromSql>(&self, column: &'a str) -> Result<T, Error> {
        let column_index = self.column_index(column)?;
        let column_value = self.get(column_index)?;
        return Ok(column_value);
    }
}
