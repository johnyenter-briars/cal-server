use chrono::{DateTime, Utc, NaiveDateTime};
use rusqlite::Row;
use serde::Serialize;
use uuid::Uuid;

use crate::models::traits::construct::ConstructableFromSql;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: Uuid,
    pub repeat_every_week: u32,
    pub repeat_on_mon: bool,
    pub repeat_on_tues: bool,
    pub repeat_on_wed: bool,
    pub repeat_on_thurs: bool,
    pub repeat_on_fri: bool,
    pub repeat_on_sat: bool,
    pub repeat_on_sun: bool,
    pub ends_on: Option<DateTime<Utc>>,
}

impl ConstructableFromSql<Series> for Series {
    fn construct(row: &Row) -> Series {
        Series {
            id: Uuid::parse_str(&row.get::<usize, String>(0).expect("no 0th row??")).expect("THis really shouldnt fail") ,
            repeat_every_week: row.get(1).expect("no 0th row??"),
            repeat_on_mon: row.get(2).expect("no 0th row??"),
            repeat_on_tues: row.get(3).expect("no 0th row??"),
            repeat_on_wed: row.get(4).expect("no 0th row??"),
            repeat_on_thurs: row.get(5).expect("no 0th row??"),
            repeat_on_fri: row.get(6).expect("no 0th row??"),
            repeat_on_sat: row.get(7).expect("no 0th row??"),
            repeat_on_sun: row.get(8).expect("no 0th row??"),
            ends_on: row.get::<usize, Option<u32>>(9).expect("idk").map(|unix_time_stamp| DateTime::from_utc(
                    NaiveDateTime::from_timestamp(unix_time_stamp as i64, 0), //this is scary
                    Utc,
                )),
        }
    }
}
