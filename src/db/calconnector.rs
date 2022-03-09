use super::DB_NAME;
use crate::models::event::Event;
use rusqlite::{Connection, params};
use std::error::Error;

pub struct CalConnector {}

impl CalConnector {
    pub fn create_event(event: Event) -> Result<bool, Box<dyn Error>> {
        let conn = Connection::open(DB_NAME)?;

        println!("{:?}", event);

        // conn.execute(
        //     "INSERT INTO events (eventtime, description) VALUES (?1, ?2)",
        //     params![event.event_time, event.description],
        // )?;

        Ok(true)
    }
}
