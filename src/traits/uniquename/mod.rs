use rusqlite::Connection;
use crate::traits::dbmodel::DbModel;
pub trait UniqueName: DbModel {
    const NAME: &'static str;
    fn get_name(&self) -> String;
}
pub trait UniqueNameModel: UniqueName {
    fn get_by_name_sql() -> String;
    fn get_by_name<'n>(c: &mut Connection, name: &'n str) -> Result<Self, rusqlite::Error>;
}
impl<T: UniqueName> UniqueNameModel for T {
    fn get_by_name_sql() -> String {
        return format!(
            "select {}.* from {} as {} where {}.{} = :name",
            T::ALIAS, T::TABLE, T::ALIAS, T::ALIAS, T::NAME
        );
    }
    fn get_by_name<'n>(c: &mut Connection, name: &'n str) -> Result<Self, rusqlite::Error> {
        let mut stmt = c.prepare(&T::get_by_name_sql())?;
        return stmt.query_row(rusqlite::named_params!{ ":name": name }, |row| {
            T::from_row(&row)
        });
    }
}
