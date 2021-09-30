use rusqlite::Connection;
pub struct DbObject {
    path: String,
    name: String,
}
impl DbObject {
    pub fn new<'a>(path: &'a str, name: &'a str) -> DbObject {
        return DbObject { path: path.to_string(), name: name.to_string(), };
    }
}
pub struct DbContext {
    connection: Connection,
    databases: Vec<DbObject>,
}
impl DbContext {
    pub fn new(c: Connection, dbs: Vec<DbObject>) -> DbContext {
        return DbContext { connection: c, databases: dbs, };
    }
    pub fn use_connection(&mut self) -> &mut Connection {
        return &mut self.connection;
    }
    pub fn attach_temp_dbs(&mut self) {
        self.databases.iter().for_each(|db| {
            match self.connection.execute(&format!("attach ':memory:' as {}", db.name), []) {
                Ok(_) => {},
                Err(e) => panic!("{}", e),
            }
        });
    }
    pub fn attach_dbs(&mut self) {
        self.databases.iter().for_each(|db| {
            match self.connection.execute(&format!("attach '{}' as {}", db.path, db.name), []) {
                Ok(_) => {},
                Err(e) => panic!("{}", e),
            }
        });
    }
}
