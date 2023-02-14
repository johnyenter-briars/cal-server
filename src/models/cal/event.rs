use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::Row;
use serde::Serialize;
use uuid::Uuid;

use crate::{models::traits::construct::ConstructableFromSql, CalResult};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Uuid,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub cal_user_id: Uuid,
    pub series_id: Option<Uuid>,
    pub calendar_id: Uuid,
    pub color: String,
    pub num_times_notified: i32,
    pub should_notify: bool,
}

impl ConstructableFromSql<Event> for Event {
    fn construct(row: &Row) -> CalResult<Self>
    where
        Self: std::marker::Sized,
    {
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
            cal_user_id: Uuid::parse_str(&row.get::<usize, String>(5)?)?,
            series_id: match row.get::<usize, String>(6) {
                Ok(string) => Some(Uuid::parse_str(&string)?),
                Err(_) => None,
            },
            calendar_id: Uuid::parse_str(&row.get::<usize, String>(7)?)?,
            color: row.get(8)?,
            num_times_notified: row.get(9)?,
            should_notify: row.get(10)?,
        })
    }
}
