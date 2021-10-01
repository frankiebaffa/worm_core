use rusqlite::Connection;
use crate::traits::{
    dbctx::DbCtx,
    dbmodel::{
        DbModel,
        AttachedDbType,
    },
};
pub trait UniqueName<T: DbCtx, A: AttachedDbType>: DbModel<T, A> {
    const NAME: &'static str;
    fn get_name(&self) -> String;
}
pub trait UniqueNameModel<T: DbCtx, A: AttachedDbType>: UniqueName<T, A> {
    fn get_by_name_sql() -> String;
    fn get_by_name<'n>(c: &mut Connection, name: &'n str) -> Result<Self, rusqlite::Error>;
}
impl<U: DbCtx, A: AttachedDbType, T: UniqueName<U, A>> UniqueNameModel<U, A> for T {
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
