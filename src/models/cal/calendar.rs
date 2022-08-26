
use rusqlite::Row;
use serde::Serialize;
use uuid::Uuid;

use crate::models::traits::construct::ConstructableFromSql;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub cal_user_id: Uuid,
    pub color: String,
}

impl ConstructableFromSql<Calendar> for Calendar {
    fn construct(row: &Row) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: std::marker::Sized,
    {
        Ok(Calendar {
            id: Uuid::parse_str(&row.get::<usize, String>(0)?)?,
            name: row.get(1)?,
            description: row.get(2)?,
            cal_user_id: Uuid::parse_str(&row.get::<usize, String>(3)?)?,
            color: row.get(4)?,
        })
    }
}
