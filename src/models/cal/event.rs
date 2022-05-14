use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::Row;
use serde::Serialize;
use uuid::Uuid;

use crate::models::traits::construct::ConstructableFromSql;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Uuid,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub caluser_id: Uuid,
    pub series_id: Option<Uuid>,
}


impl ConstructableFromSql<Event> for Event {
    fn construct(row: &Row) -> Result<Self, Box<dyn std::error::Error>> where Self: std::marker::Sized {
        Ok(Event {
            id: Uuid::parse_str(&row.get::<usize, String>(0)?)?,
            start_time: match row.get(1) {
                Ok(unix_time_stamp) => Some(DateTime::from_utc(
                    NaiveDateTime::from_timestamp(unix_time_stamp, 0),
                    Utc,
                )),
                Err(_) => None,
            },
            end_time: match row.get(2) {
                Ok(unix_time_stamp) => Some(DateTime::from_utc(
                    NaiveDateTime::from_timestamp(unix_time_stamp, 0),
                    Utc,
                )),
                Err(_) => None,
            },
            name: row.get(3)?,
            description: row.get(4)?,
            caluser_id: Uuid::parse_str(&row.get::<usize, String>(5)?)? ,
            series_id:  match row.get::<usize, String>(6) {
                Ok(string) => Some(Uuid::parse_str(&string)?),
                Err(_) => None,
            },
        })
    }
}
