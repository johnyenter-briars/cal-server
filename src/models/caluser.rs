use rusqlite::Row;
use serde::Serialize;
use super::construct::ConstructableFromSql;
#[derive(Serialize)]
pub struct CalUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl ConstructableFromSql<CalUser> for CalUser {
    fn construct(row: &Row) -> CalUser {
        CalUser {
            id: row.get(0).expect("no 0th row??"),
            first_name: row.get(1).expect("no 1th row??"),
            last_name: row.get(2).expect("no 2th row??"),
        }
    }
}
