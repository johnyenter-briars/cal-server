use rusqlite::Row;
use serde::Serialize;
use uuid::Uuid;
use crate::models::traits::construct::ConstructableFromSql;
// use super::construct::ConstructableFromSql;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

impl ConstructableFromSql<CalUser> for CalUser {
    fn construct(row: &Row) -> CalUser {
        CalUser {
            id: Uuid::parse_str(&row.get::<usize, String>(0).expect("no 0th row??")).expect("THis really shouldnt fail") ,
            first_name: row.get(1).expect("no 1th row??"),
            last_name: row.get(2).expect("no 2th row??"),
        }
    }
}
