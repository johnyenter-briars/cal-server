use rusqlite::Row;
use serde::Serialize;
use uuid::Uuid;

use crate::{models::traits::construct::ConstructableFromSql, CalResult};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedCalendar {
    pub id: Uuid,
    pub calendar_id: Uuid,
    pub owner_cal_user_id: Uuid,
    pub cal_user_id: Uuid,
}

impl ConstructableFromSql<SharedCalendar> for SharedCalendar {
    fn construct(row: &Row) -> CalResult<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(SharedCalendar {
            id: Uuid::parse_str(&row.get::<usize, String>(0)?)?,
            calendar_id: Uuid::parse_str(&row.get::<usize, String>(1)?)?,
            owner_cal_user_id: Uuid::parse_str(&row.get::<usize, String>(2)?)?,
            cal_user_id: Uuid::parse_str(&row.get::<usize, String>(3)?)?,
        })
    }
}
