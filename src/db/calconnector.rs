use super::DB_NAME;
use crate::models::{
    caluser::CalUser, construct::ConstructableFromSql, event::Event,
    requests::createeventrequest::CreateEventRequest,
};
use rusqlite::{params, Connection};
use std::{
    error::Error,
};

pub struct CalConnector {}

impl CalConnector {
    pub fn create_caluser(first_name: &str, last_name: &str) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open(DB_NAME)?;

        conn.execute(
            &format!("insert into caluser (firstname, lastname) values ('{first_name}', '{last_name}');"),
            [],
        )?;

        Ok(())
    }

    pub fn get_caluser(id: u32) -> Result<CalUser, Box<dyn Error>> {
        let mut users = CalConnector::get_records::<CalUser>(&format!(
            "SELECT id, firstname, lastname FROM caluser where id = {id}"
        ))?;

        assert!(users.len() == 1, "More than one users with that id! : (");

        match users.pop() {
            Some(u) => Ok(u),
            _ => unreachable!(),
        }
    }

    pub fn create_event(event_req: CreateEventRequest) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open(DB_NAME)?;

        conn.execute(
            "INSERT INTO event (time, name, caluserid) VALUES (?1, ?2, ?3)",
            params![event_req.time.timestamp(), event_req.name, event_req.cal_user_id],
        )?;

        Ok(())
    }

    pub fn get_events() -> Result<Vec<Event>, Box<dyn Error>> {
        Ok(CalConnector::get_records::<Event>(
            "SELECT id, time, name, caluserid name FROM event",
        )?)
    }

    fn get_records<T>(sql: &str) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: ConstructableFromSql<T>,
    {
        let conn = Connection::open(DB_NAME)?;

        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| Ok(T::construct(row)))?;

        Ok(rows.filter_map(|e| e.ok()).collect())
    }
}
