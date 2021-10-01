use crate::traits::{
    dbctx::DbCtx,
    dbmodel::AttachedDbType,
    primarykey::PrimaryKeyModel,
};
use rusqlite::Error;
pub trait ForeignKey<T: DbCtx, A: AttachedDbType, U: PrimaryKeyModel<T, A>>: PrimaryKeyModel<T, A> {
    const FOREIGN_KEY: &'static str;
    const FOREIGN_KEY_PARAM: &'static str;
    fn get_fk_value(&self) -> i64;
}
pub trait ForeignKeyModel<T: DbCtx, A: AttachedDbType, U: PrimaryKeyModel<T, A>>: ForeignKey<T, A, U> {
    fn get_all_by_fk_sql() -> String;
    fn get_fk_sql() -> String;
    fn get_all_by_fk(c: &mut impl DbCtx, references: U) -> Result<Vec<Self>, Error>;
    fn get_fk(&self, c: &mut impl DbCtx) -> Result<U, Error>;
}
impl<V: DbCtx, A: AttachedDbType, U: PrimaryKeyModel<V, A>, T: ForeignKey<V, A, U>> ForeignKeyModel<V, A, U> for T {
    fn get_all_by_fk_sql() -> String {
        return format!(
            "select {}.* from {} as {} where {}.{} = {};",
            T::ALIAS, T::TABLE, T::ALIAS, T::ALIAS, T::FOREIGN_KEY, T::FOREIGN_KEY_PARAM
        );
    }
    fn get_fk_sql() -> String {
        return format!(
            "select {}.* from {} as {} join {} as {} on {}.{} = {}.{} and {}.{} = {}",
            T::ALIAS, T::TABLE, T::ALIAS, U::TABLE, U::ALIAS, T::ALIAS, T::FOREIGN_KEY, U::ALIAS, U::PRIMARY_KEY, U::ALIAS, U::PRIMARY_KEY, T::FOREIGN_KEY_PARAM
        );
    }
    fn get_all_by_fk(db: &mut impl DbCtx, references: U) -> Result<Vec<Self>, Error> {
        let c = db.use_connection();
        let mut stmt = c.prepare(&Self::get_all_by_fk_sql())?;
        return stmt.query_map(&[(Self::FOREIGN_KEY_PARAM, &references.get_id())], |row| {
            Self::from_row(&row)
        })?.into_iter().collect();
    }
    fn get_fk(&self, db: &mut impl DbCtx) -> Result<U, Error> {
        let c = db.use_connection();
        let mut stmt = c.prepare(&Self::get_fk_sql())?;
        return Ok(stmt.query_row(&[(Self::FOREIGN_KEY_PARAM, &self.get_fk_value())], |row| {
            U::from_row(&row)
        })?);
    }
}
