use crate::traits::{
    dbctx::DbCtx,
    dbmodel::{
        AttachedDbType,
        DbModel,
    },
};
pub trait PrimaryKey<T: DbCtx, A: AttachedDbType>: DbModel<T, A> {
    const PRIMARY_KEY: &'static str;
    fn get_id(&self) -> i64;
}
pub trait PrimaryKeyModel<T: DbCtx, A: AttachedDbType>: PrimaryKey<T, A> {
    fn get_by_id_sql() -> String;
    fn get_by_id(c: &mut T, id: i64) -> Result<Self, rusqlite::Error>;
}
impl<U: DbCtx, A: AttachedDbType, T: PrimaryKey<U, A>> PrimaryKeyModel<U, A> for T {
    fn get_by_id_sql() -> String {
        return format!(
            "select {}.* from {} as {} where {}.{} = :id;",
            T::ALIAS, T::TABLE, T::ALIAS, T::ALIAS, T::PRIMARY_KEY
        );
    }
    fn get_by_id(db: &mut U, id: i64) -> Result<Self, rusqlite::Error> {
        let c = db.use_connection();
        let mut stmt = c.prepare(&Self::get_by_id_sql())?;
        return stmt.query_row(&[(":id", &id)], |row| {
            Self::from_row(&row)
        });
    }
}
