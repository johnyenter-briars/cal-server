use crate::{models::traits::construct::ConstructableFromSql, CalResult};
use rusqlite::Row;
use serde::Serialize;
use uuid::Uuid;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub api_key: String,
}

impl ConstructableFromSql<CalUser> for CalUser {
    fn construct(row: &Row) -> CalResult<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(CalUser {
            id: Uuid::parse_str(&row.get::<usize, String>(0)?)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            api_key: row.get(3)?,
        })
    }
}
