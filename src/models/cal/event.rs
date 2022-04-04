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
    pub caluser_id: Uuid,
    pub series_id: Option<Uuid>,
}

impl Event {
    pub fn new(
        id: Uuid,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        name: String,
        cal_user_id: Uuid,
        series_id: Option<Uuid>,
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
        // panic!("");
        Event {
            id: Uuid::parse_str(&row.get::<usize, String>(0).expect("no 0th row??")).expect("THis really shouldnt fail") ,
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
            name: row.get(3).expect("no 2th row??"),
            caluser_id: Uuid::parse_str(&row.get::<usize, String>(4).expect("no 0th row??")).expect("THis really shouldnt fail") ,
            series_id:  match row.get::<usize, String>(5) {
                Ok(x) => Some(Uuid::parse_str(&x).expect("plz")),
                Err(_) => None,
            },
        }
    }
}
