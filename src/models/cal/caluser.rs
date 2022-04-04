use rusqlite::Row;
use serde::Serialize;
use crate::models::traits::construct::ConstructableFromSql;
// use super::construct::ConstructableFromSql;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalUser {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
}

impl ConstructableFromSql<CalUser> for CalUser {
    fn construct(row: &Row) -> CalUser {
        let idk: i32 = match row.get(0){
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                5
            }
        };
        // let idk = row.get(0);
        // let idk = row.get(0);
        CalUser {
            id: row.get(0).expect("no 0th row??"),
            first_name: row.get(1).expect("no 1th row??"),
            last_name: row.get(2).expect("no 2th row??"),
        }
    }
}
