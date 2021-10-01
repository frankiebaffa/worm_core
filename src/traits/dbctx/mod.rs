use rusqlite::Connection;
pub trait DbCtx: Sized {
    fn init() -> Self;
    fn use_connection(&mut self) -> &mut Connection;
}
