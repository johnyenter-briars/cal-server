use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::Row;
use serde::Serialize;

use crate::models::traits::construct::ConstructableFromSql;


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: u32,
    pub time: DateTime<Utc>,
    pub name: String,
    pub caluser_id: u32,
}

impl Event {
    pub fn new(id: u32, time: DateTime<Utc>, name: String, cal_user_id: u32) -> Self {
        Event { id, time, name, caluser_id: cal_user_id}
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
            caluser_id: row.get(3).expect("no 3th row??"),
        }
    }
}
