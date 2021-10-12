use crate::DbCtx;
use rusqlite::{named_params, ToSql};
struct Query<'a, T> {
    select: String,
    join: String,
    clause: String,
}
impl<T> Query<T> {
    fn execute<'a>(&mut self, db: &'a mut impl DbCtx) -> Result<Vec<T>, rusqlite::Error> {
        let sql = format!("{}\r\n{}\r\n{}", self.select, self.join, self.clause);
        let c = db.use_connection();
        let mut stmt = c.prepare(&sql)?;
        let mut params;

    }
}
