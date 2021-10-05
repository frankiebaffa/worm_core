use crate::traits::{
    primarykey::PrimaryKeyModel,
    dbctx::DbCtx,
};
use rusqlite::named_params;
pub trait ActiveFlag: PrimaryKeyModel {
    const ACTIVE: &'static str;
    fn get_active(&self) -> bool;
}
pub trait ActiveFlagModel: ActiveFlag {
    fn get_all_active_sql() -> String;
    fn get_all_active(db: &mut impl DbCtx) -> Result<Vec<Self>, rusqlite::Error>;
    fn deactivate(&self, db: &mut impl DbCtx) -> Result<Self, rusqlite::Error>;
    fn activate(&self, db: &mut impl DbCtx) -> Result<Self, rusqlite::Error>;
    fn flip_active(&self, db: &mut impl DbCtx, active: bool) -> Result<Self, rusqlite::Error>;
}
impl<T: ActiveFlag> ActiveFlagModel for T {
    fn get_all_active_sql() -> String {
        return format!(
            "select {}.* from {} as {} where {}.{} = 1;",
            T::ALIAS, T::TABLE, T::ALIAS, T::ALIAS, T::ACTIVE
        );
    }
    fn get_all_active(db: &mut impl DbCtx) -> Result<Vec<T>, rusqlite::Error> {
        let c = db.use_connection();
        let mut stmt = c.prepare(&T::get_all_active_sql())?;
        return stmt.query_map([], |row| {
            T::from_row(&row)
        })?.into_iter().collect();
    }
    fn flip_active(&self, db: &mut impl DbCtx, active: bool) -> Result<Self, rusqlite::Error> {
        let sql = format!(
            "update {}.{} set {} = :active where {} = :id",
            T::DB, T::TABLE, T::ACTIVE, T::PRIMARY_KEY,
        );
        let id;
        {
            let c = db.use_connection();
            {
                let mut tx = c.transaction()?;
                {
                    let sp = tx.savepoint()?;
                    sp.execute(&sql, named_params!{ ":id": self.get_id(), ":active": active })?;
                    id = sp.last_insert_rowid();
                    sp.commit()?;
                }
                tx.commit()?;
            }
        }
        return Self::get_by_id(db, id);
    }
    fn deactivate(&self, db: &mut impl DbCtx) -> Result<Self, rusqlite::Error> {
        return self.flip_active(db, false);
    }
    fn activate(&self, db: &mut impl DbCtx) -> Result<Self, rusqlite::Error> {
        return self.flip_active(db, true);
    }
}
