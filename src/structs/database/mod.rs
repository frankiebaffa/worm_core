use rusqlite::{
    Connection,
    Error,
};
pub struct DbObject {
    path: String,
    name: String,
}
impl DbObject {
    pub fn new<'a>(path: &'a str, name: &'a str) -> DbObject {
        return DbObject { path: path.to_string(), name: name.to_string(), };
    }
    pub fn get_db_file_path(&self) -> String {
        if self.path.ends_with("/") {
            return format!("{}{}.db", self.path, self.name);
        } else {
            return format!("{}/{}.db", self.path, self.name);
        }
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
    pub fn attach_temp_dbs(self) -> Result<(), Error> {
        for db in self.databases {
            self.connection.execute(&format!("attach ':memory:' as {}", db.name), [])?;
        }
        return Ok(());
    }
    pub fn attach_dbs(self) -> Result<(), Error> {
        for db in self.databases {
            self.connection.execute(&format!("attach '{}' as {}", db.get_db_file_path(), db.name), [])?;
        }
        return Ok(());
    }
}
