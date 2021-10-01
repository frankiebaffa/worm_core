use crate::traits::{
    dbmodel::{
        DbModel,
        AttachedDbType,
    },
    dbctx::DbCtx,
};
use rusqlite::Connection;
pub trait ActiveFlag<T: DbCtx, A: AttachedDbType>: DbModel<T, A> {
    const ACTIVE: &'static str;
    fn get_active(&self) -> bool;
}
pub trait ActiveFlagModel<T: DbCtx, A: AttachedDbType>: ActiveFlag<T, A> {
    fn get_all_active_sql() -> String;
    fn get_all_active(c: &mut Connection) -> Result<Vec<Self>, rusqlite::Error>;
}
impl<U: DbCtx, A: AttachedDbType, T: ActiveFlag<U, A>> ActiveFlagModel<U, A> for T {
    fn get_all_active_sql() -> String {
        return format!(
            "select {}.* from {} as {} where {}.{} = 1;",
            T::ALIAS, T::TABLE, T::ALIAS, T::ALIAS, T::ACTIVE
        );
    }
    fn get_all_active(c: &mut Connection) -> Result<Vec<T>, rusqlite::Error> {
        let mut stmt = c.prepare(&T::get_all_active_sql())?;
        return stmt.query_map([], |row| {
            T::from_row(&row)
        })?.into_iter().collect();
    }
}
