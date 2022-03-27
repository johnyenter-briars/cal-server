use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::Row;
use serde::Serialize;

use super::construct::ConstructableFromSql;

#[derive(Debug, Serialize)]
pub struct Event {
    pub id: i32,
    pub time: DateTime<Utc>,
    pub name: String,
}

impl Event {
    pub fn new(time: DateTime<Utc>, name: String) -> Self {
        Event { id: 0, time, name }
    }
}

impl ConstructableFromSql<Event> for Event {
    fn construct(row: &Row) -> Event {
        Event {
            id: row.get(0).expect("no 0th row??"),
            time: DateTime::from_utc(
                NaiveDateTime::from_timestamp(row.get::<usize, i64>(1).expect("Yea idk"), 0),
                Utc,
            ),
            name: row.get(2).expect("no 2th row??"),
        }
    }
}
