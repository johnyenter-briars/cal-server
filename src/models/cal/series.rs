use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use rusqlite::Row;
use serde::Serialize;
use serde_with::serde_as;
use uuid::Uuid;

use crate::{models::traits::construct::ConstructableFromSql, CalResult};

#[serde_as]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
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
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub event_start_time: Duration,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub event_end_time: Duration,
    pub cal_user_id: Uuid,
    pub calendar_id: Uuid,
    pub color: String,
    pub num_times_notified: i32,
    pub should_notify: bool,
}

impl ConstructableFromSql<Series> for Series {
    fn construct(row: &Row) -> CalResult<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(Series {
            id: Uuid::parse_str(&row.get::<usize, String>(0)?)?,
            name: row.get(1)?,
            description: row.get(2)?,
            repeat_every_week: row.get(3)?,
            repeat_on_mon: row.get(4)?,
            repeat_on_tues: row.get(5)?,
            repeat_on_wed: row.get(6)?,
            repeat_on_thurs: row.get(7)?,
            repeat_on_fri: row.get(8)?,
            repeat_on_sat: row.get(9)?,
            repeat_on_sun: row.get(10)?,
            starts_on: row.get::<usize, Option<u32>>(11)?.map(|unix_time_stamp| {
                DateTime::from_utc(
                    NaiveDateTime::from_timestamp(unix_time_stamp as i64, 0), //this is scary
                    Utc,
                )
            }),
            ends_on: row.get::<usize, Option<u32>>(12)?.map(|unix_time_stamp| {
                DateTime::from_utc(
                    NaiveDateTime::from_timestamp(unix_time_stamp as i64, 0), //this is scary
                    Utc,
                )
            }),
            event_start_time: Duration::seconds(row.get::<usize, i64>(13)?),
            event_end_time: Duration::seconds(row.get::<usize, i64>(14)?),
            cal_user_id: Uuid::parse_str(&row.get::<usize, String>(15)?)?,
            calendar_id: Uuid::parse_str(&row.get::<usize, String>(16)?)?,
            color: row.get(17)?,
            num_times_notified: row.get(18)?,
            should_notify: row.get(9)?,
        })
    }
}
