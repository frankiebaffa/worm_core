use crate::traits::{
    dbctx::DbCtx,
    primarykey::PrimaryKeyModel,
};
use rusqlite::Error;
pub trait ForeignKey<U: PrimaryKeyModel>: PrimaryKeyModel {
    const FOREIGN_KEY: &'static str;
    const FOREIGN_KEY_PARAM: &'static str;
    fn get_fk_value(&self) -> i64;
}
impl<T: PrimaryKeyModel, U: ForeignKey<T>> ForeignKey<U> for T {
    const FOREIGN_KEY: &'static str = T::PRIMARY_KEY;
    const FOREIGN_KEY_PARAM: &'static str = U::FOREIGN_KEY_PARAM;
    fn get_fk_value(&self) -> i64 {
        self.get_id()
    }
}
pub trait ForeignKeyModel<T: PrimaryKeyModel, U: PrimaryKeyModel>: ForeignKey<T> {
    fn get_all_by_fk(db: &mut impl DbCtx, references: &U) -> Result<Vec<Self>, Error>;
    fn get_fk(&self, db: &mut impl DbCtx) -> Result<U, Error>;
}
impl<S: PrimaryKeyModel, T: ForeignKey<S>, U: PrimaryKeyModel> ForeignKeyModel<S, U> for T {
    fn get_all_by_fk(db: &mut impl DbCtx, references: &U) -> Result<Vec<Self>, Error> {
        let sql = format!("
            select {}.*
            from {}.{} as {}
            where {}.{} = {};",
            Self::ALIAS,
            Self::DB, Self::TABLE, Self::ALIAS,
            Self::ALIAS, Self::FOREIGN_KEY, Self::FOREIGN_KEY_PARAM
        );
        let c = db.use_connection();
        let mut stmt = c.prepare(&sql)?;
        return stmt.query_map(&[(Self::FOREIGN_KEY_PARAM, &references.get_id())], |row| {
            Self::from_row(&row)
        })?.into_iter().collect();
    }
    fn get_fk(&self, db: &mut impl DbCtx) -> Result<U, Error> {
        let sql = format!("
            select {}.*
            from {}.{} as {}
            join {}.{} as {}
            on {}.{} = {}.{}
            and {}.{} = {}",
            U::ALIAS,
            Self::DB, Self::TABLE, Self::ALIAS,
            U::DB, U::TABLE, U::ALIAS,
            Self::ALIAS, Self::FOREIGN_KEY, U::ALIAS, U::PRIMARY_KEY,
            U::ALIAS, U::PRIMARY_KEY, Self::FOREIGN_KEY_PARAM
        );
        let c = db.use_connection();
        let mut stmt = c.prepare(&sql)?;
        return Ok(stmt.query_row(&[(Self::FOREIGN_KEY_PARAM, &self.get_fk_value())], |row| {
            U::from_row(&row)
        })?);
    }
}
