use std::error::Error;

use rusqlite::{params, Connection};

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("DB connection failed");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY,
            name TEXT,
            price TEXT,
            img TEXT,
            url TEXT UNIQUE)",
            [],
        )
        .expect("Table creation failed");

        Db { conn }
    }
    pub fn insert_entry(
        &self,
        name: &str,
        price: &str,
        img: &str,
        url: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.conn.execute(
            "INSERT OR IGNORE INTO entries (name, price, img, url) VALUES (?, ?, ?, ?)",
            params![name, price, img, url],
        )?;
        Ok(())
    }
}
