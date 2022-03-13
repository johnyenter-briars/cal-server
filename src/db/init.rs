use rusqlite::{Connection, Result};
use super::DB_NAME;

pub fn initiaize_db() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS event (
                  id              INTEGER PRIMARY KEY,
                  time            INTEGER,
                  name            TEXT NOT NULL
                  )",
        [],
    )?;

    conn.execute(
        " delete from event
                  ",
        [],
    )?;

    match conn.close() {
        Ok(_) => Ok(()),
        Err((_, err)) => Err(Box::new(err)),
    }
}
