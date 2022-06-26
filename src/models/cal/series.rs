use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::Row;
use serde::Serialize;
use uuid::Uuid;

use crate::models::traits::construct::ConstructableFromSql;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub repeat_every_week: u32,
    pub repeat_on_mon: bool,
    pub repeat_on_tues: bool,
    pub repeat_on_wed: bool,
    pub repeat_on_thurs: bool,
    pub repeat_on_fri: bool,
    pub repeat_on_sat: bool,
    pub repeat_on_sun: bool,
    pub starts_on: Option<DateTime<Utc>>,
    pub ends_on: Option<DateTime<Utc>>,
}

impl ConstructableFromSql<Series> for Series {
    fn construct(row: &Row) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: std::marker::Sized,
    {
        Ok(Series {
            id: Uuid::parse_str(&row.get::<usize, String>(0)?)?,
            repeat_every_week: row.get(1)?,
            repeat_on_mon: row.get(2)?,
            repeat_on_tues: row.get(3)?,
            repeat_on_wed: row.get(4)?,
            repeat_on_thurs: row.get(5)?,
            repeat_on_fri: row.get(6)?,
            repeat_on_sat: row.get(7)?,
            repeat_on_sun: row.get(8)?,
            starts_on: row.get::<usize, Option<u32>>(9)?.map(|unix_time_stamp| {
                DateTime::from_utc(
                    NaiveDateTime::from_timestamp(unix_time_stamp as i64, 0), //this is scary
                    Utc,
                )
            }),
            ends_on: row.get::<usize, Option<u32>>(10)?.map(|unix_time_stamp| {
                DateTime::from_utc(
                    NaiveDateTime::from_timestamp(unix_time_stamp as i64, 0), //this is scary
                    Utc,
                )
            }),
            name: row.get(11)?,
            description: row.get(12)?,
        })
    }
}
