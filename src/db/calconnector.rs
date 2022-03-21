use super::DB_NAME;
use crate::models::event::Event;
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{params, Connection};
use std::error::Error;

pub struct CalConnector {}

impl CalConnector {
    pub fn create_event(event: Event) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open(DB_NAME)?;

        conn.execute(
            "INSERT INTO event (time, name) VALUES (?1, ?2)",
            params![event.time.timestamp(), event.name],
        )?;

        Ok(())
    }

    pub fn get_events() -> Result<Vec<Event>, Box<dyn Error>> {
        let conn = Connection::open(DB_NAME)?;

        let mut stmt = conn.prepare("SELECT id, time, name FROM event")?;
        let event_iter = stmt.query_map([], |row| {
            Ok(Event {
                id: row.get(0)?,
                time: DateTime::from_utc(
                    NaiveDateTime::from_timestamp(row.get::<usize, i64>(1)?, 0),
                    Utc,
                ),
                name: row.get(2)?,
            })
        })?;

        Ok(event_iter.filter_map(|e| e.ok()).collect())
    }
}
