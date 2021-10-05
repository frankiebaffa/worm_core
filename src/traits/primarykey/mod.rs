use crate::traits::{
    dbctx::DbCtx,
    dbmodel::DbModel,
};
pub trait PrimaryKey: DbModel {
    const PRIMARY_KEY: &'static str;
    fn get_id(&self) -> i64;
}
pub trait PrimaryKeyModel: PrimaryKey {
    fn get_by_id(c: &mut impl DbCtx, id: i64) -> Result<Self, rusqlite::Error>;
}
impl<T: PrimaryKey> PrimaryKeyModel for T {
    fn get_by_id(db: &mut impl DbCtx, id: i64) -> Result<Self, rusqlite::Error> {
        let sql = format!(
            "select {}.* from {} as {} where {}.{} = :id;",
            T::ALIAS, T::TABLE, T::ALIAS, T::ALIAS, T::PRIMARY_KEY
        );
        let c = db.use_connection();
        let mut stmt = c.prepare(&sql)?;
        return stmt.query_row(&[(":id", &id)], |row| {
            Self::from_row(&row)
        });
    }
}
