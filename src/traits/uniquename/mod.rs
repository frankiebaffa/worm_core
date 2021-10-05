use crate::traits::{
    dbctx::DbCtx,
    activeflag::ActiveFlag,
    helpers::ColumnValue,
};
pub trait UniqueName: ActiveFlag {
    const NAME: &'static str;
    fn get_name(&self) -> String;
}
pub trait UniqueNameModel: UniqueName {
    fn get_by_name_sql() -> String;
    fn get_by_name<'n>(c: &mut impl DbCtx, name: &'n str) -> Result<Self, rusqlite::Error>;
    fn get_all_names_sql() -> String;
    fn get_all_names(db: &mut impl DbCtx) -> Result<Vec<String>, rusqlite::Error>;
}
impl<T: UniqueName> UniqueNameModel for T {
    fn get_by_name_sql() -> String {
        return format!(
            "select {}.* from {}.{} as {} where {}.{} = :name",
            T::ALIAS, T::DB, T::TABLE, T::ALIAS, T::ALIAS, T::NAME
        );
    }
    fn get_by_name<'n>(db: &mut impl DbCtx, name: &'n str) -> Result<Self, rusqlite::Error> {
        let sql = T::get_by_name_sql();
        let c = db.use_connection();
        let mut stmt = c.prepare(&sql)?;
        return stmt.query_row(rusqlite::named_params!{ ":name": name }, |row| {
            T::from_row(&row)
        });
    }
    fn get_all_names_sql() -> String {
        return format!(
            "select {}.{} from {}.{} as {} where {}.{} = 1",
            T::ALIAS, T::NAME, T::DB, T::TABLE, T::ALIAS, T::ALIAS, T::ACTIVE,
        );
    }
    fn get_all_names(db: &mut impl DbCtx) -> Result<Vec<String>, rusqlite::Error> {
        let sql = Self::get_all_names_sql();
        let c = db.use_connection();
        let mut stmt = c.prepare(&sql)?;
        let names_res: Vec<Result<String, rusqlite::Error>> = stmt.query_map([], |row| {
            row.value("Name")
        })?.collect();
        let mut names = Vec::new();
        for name_res in names_res {
            match name_res {
                Ok(name) => names.push(name),
                Err(e) => return Err(e),
            }
        }
        return Ok(names);
    }
}
