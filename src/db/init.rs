use rusqlite::{params, Connection, Result};
use super::DB_NAME;

pub fn initiaize_db() -> Result<bool, Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )?;

    conn.execute(
        " delete from person
                  ",
        [],
    )?;

    match conn.close() {
        Ok(_) => Ok(true),
        Err((_, err)) => Err(Box::new(err)),
    }
}
