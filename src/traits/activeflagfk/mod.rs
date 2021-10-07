use {
    crate::traits::{
        dbctx::DbCtx,
        activeflag::ActiveFlagModel,
        foreignkey::ForeignKey,
        foreignkey::ForeignKeyModel,
        primarykey::PrimaryKeyModel,
    },
    rusqlite::Error,
};
pub trait FKActiveFlagModel<T: PrimaryKeyModel, U: PrimaryKeyModel, V: ForeignKey<T>>: ForeignKeyModel<V, U> + ActiveFlagModel {
    fn get_all_by_active_fk(db: &mut impl DbCtx, references: &U) -> Result<Vec<Self>, Error>;
}
impl<T: PrimaryKeyModel, U: PrimaryKeyModel, V: ForeignKey<T>, W: ForeignKeyModel<V, U> + ActiveFlagModel> FKActiveFlagModel<T, U, V> for W {
    fn get_all_by_active_fk(db: &mut impl DbCtx, references: &U) -> Result<Vec<Self>, Error> {
        let sql = format!("
            select {}.*
            from {}.{} as {}
            where {}.{} = {}
            and {}.{} = 1;",
            Self::ALIAS,
            Self::DB, Self::TABLE, Self::ALIAS,
            Self::ALIAS, Self::FOREIGN_KEY, Self::FOREIGN_KEY_PARAM,
            Self::ALIAS, Self::ACTIVE,
        );
        let c = db.use_connection();
        let mut stmt = c.prepare(&sql)?;
        return stmt.query_map(&[(Self::FOREIGN_KEY_PARAM, &references.get_id())], |row| {
            Self::from_row(&row)
        })?.into_iter().collect();
    }
}
pub trait ActiveFlagFKModel<T: PrimaryKeyModel, U: PrimaryKeyModel + ActiveFlagModel, V: ForeignKey<T>>: ForeignKeyModel<V, U> {
    fn get_active_fk(&self, db: &mut impl DbCtx) -> Result<U, Error>;
}
impl<T: PrimaryKeyModel, U: PrimaryKeyModel + ActiveFlagModel, V: ForeignKey<T>, W: ForeignKeyModel<V, U>> ActiveFlagFKModel<T, U, V> for W {
    fn get_active_fk(&self, db: &mut impl DbCtx) -> Result<U, Error> {
        let sql = format!("
            select {}.*
            from {}.{} as {}
            join {}.{} as {}
            on {}.{} = {}.{}
            and {}.{} = {}
            and {}.{} = 1;",
            U::ALIAS,
            Self::DB, Self::TABLE, Self::ALIAS,
            U::DB, U::TABLE, U::ALIAS,
            Self::ALIAS, Self::FOREIGN_KEY, U::ALIAS, U::PRIMARY_KEY,
            U::ALIAS, U::PRIMARY_KEY, Self::FOREIGN_KEY_PARAM,
            U::ALIAS, U::ACTIVE
        );
        let c = db.use_connection();
        let mut stmt = c.prepare(&sql)?;
        return Ok(stmt.query_row(&[(Self::FOREIGN_KEY_PARAM, &self.get_fk_value())], |row| {
            U::from_row(&row)
        })?);
    }
}
