use rusqlite::Connection;
pub trait DbCtx: Sized {
    fn init() -> Self;
    fn use_connection(&mut self) -> &mut Connection;
    fn attach_temp_dbs(&mut self);
    fn attach_dbs(&mut self);
    fn delete_db_files(&mut self) -> Result<(), String>;
}
