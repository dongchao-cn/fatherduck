use duckdb::Connection;

pub struct MyConnection {
    conn: Connection
}

impl MyConnection {
    pub fn new(conn: Connection) -> MyConnection {

        MyConnection {
            conn
        }
    }

    pub fn get(&self) -> &Connection {
        &self.conn
    }

}
unsafe impl Sync for MyConnection {}
unsafe impl Send for MyConnection {}
