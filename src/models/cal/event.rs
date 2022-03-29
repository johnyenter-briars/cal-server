use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::Row;
use serde::Serialize;

use crate::models::traits::construct::ConstructableFromSql;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: u32,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub name: String,
    pub caluser_id: u32,
    pub series_id: Option<u32>,
}

impl Event {
    pub fn new(
        id: u32,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        name: String,
        cal_user_id: u32,
        series_id: Option<u32>,
    ) -> Self {
        Event {
            id,
            start_time,
            end_time,
            name,
            caluser_id: cal_user_id,
            series_id,
        }
    }
}

impl ConstructableFromSql<Event> for Event {
    fn construct(row: &Row) -> Event {
        // Event {
        //     id: row.get("id").expect("no 0th row??"),
        //     // start_time: DateTime::from_utc(
        //     //     NaiveDateTime::from_timestamp(row.get::<usize, i64>(1).expect("Yea idk"), 0),
        //     //     Utc,
        //     // ),
        //     start_time: match row.get("") {
        //         Ok(unix_time_stamp) => Some(DateTime::from_utc(
        //             NaiveDateTime::from_timestamp(unix_time_stamp, 0),
        //             Utc,
        //         )),
        //         Err(_) => None,
        //     },
        //     end_time: Some(DateTime::from_utc(
        //         NaiveDateTime::from_timestamp(row.get::<usize, i64>(2).expect("Yea idk"), 0),
        //         Utc,
        //     )),
        //     name: row.get(3).expect("no 2th row??"),
        //     caluser_id: row.get(4).expect("no 3th row??"),
        //     series_id: row.get(5).expect("no 3th row??"),
        // }
        panic!("")
    }
}
