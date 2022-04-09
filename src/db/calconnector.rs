use super::DB_NAME;
use crate::models::{
    cal::{caluser::CalUser, event::Event, series::Series},
    server::requests::{
        createeventrequest::CreateEventRequest, createseriesrequest::CreateSeriesRequest,
    },
    traits::construct::ConstructableFromSql,
};
use rusqlite::{params, Connection};
use uuid::Uuid;
use std::error::Error;

pub struct CalConnector {}

impl CalConnector {
    pub fn create_caluser(first_name: &str, last_name: &str, id: Option<Uuid>) -> Result<Uuid, Box<dyn Error>> {
        let new_id = match id {
            Some(id) => id,
            None =>  CalConnector::generate_random_id(),
        };
        
        println!("{}", new_id.to_string());
        let conn = Connection::open(DB_NAME)?;

        conn.execute(
            &format!(
                "insert into caluser (id, firstname, lastname) values (?1, ?2, ?3);"
            ),
            params![new_id.to_string(), first_name, last_name],
        )?;

        Ok(new_id)
    }

    pub fn create_event(event_req: CreateEventRequest) -> Result<Uuid, Box<dyn Error>> {
        if (event_req.start_time.is_none() && event_req.end_time.is_some())
            || (event_req.start_time.is_some() && event_req.end_time.is_none())
        {
            return Err(Box::from(
                "You must construct an event with both start and end either both Some or None",
            ));
        }

        let new_id = CalConnector::generate_random_id();
        let conn = Connection::open(DB_NAME)?;

        conn.execute(
            "INSERT INTO event (id, starttime, endtime, name, caluserid, seriesid) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                new_id.to_string(),
                match event_req.start_time {
                    Some(t) => Some(t.timestamp()),
                    None => None,
                },
                match event_req.end_time {
                    Some(t) => Some(t.timestamp()),
                    None => None,
                },
                event_req.name,
                event_req.cal_user_id.to_string(),
                match event_req.series_id {
                    Some(t) => Some(t.to_string()),
                    None => None,
                },
            ],
        )?;

        Ok(new_id)
    }

    pub fn create_series(series_req: CreateSeriesRequest) -> Result<Uuid, Box<dyn Error>> {
        let new_id = CalConnector::generate_random_id();
        let conn = Connection::open(DB_NAME)?;

        conn.execute(
            "INSERT INTO series (
                id,
                repeateveryweek,
                repeateonmon,
                repeateontues,
                repeateonwed,
                repeateonthurs,
                repeateonfri,
                repeateonsat,
                repeateonsun,
                endson
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                new_id.to_string(),
                series_req.repeat_every_week,
                series_req.repeat_on_mon,
                series_req.repeat_on_tues,
                series_req.repeat_on_wed,
                series_req.repeat_on_thurs,
                series_req.repeat_on_fri,
                series_req.repeat_on_sat,
                series_req.repeat_on_sun,
                match series_req.ends_on {
                    Some(t) => Some(t.timestamp()),
                    None => None,
                },
            ],
        )?;

        Ok(new_id)
    }

    pub fn get_caluser(id: Uuid) -> Result<CalUser, Box<dyn Error>> {
        let mut users = CalConnector::get_records::<CalUser>(&format!(
            "SELECT id, firstname, lastname FROM caluser where id = '{id}'"
        ))?;

        assert!(users.len() <= 1, "More than one users with that id! : (");

        match users.pop() {
            Some(u) => Ok(u),
            None => Err(Box::from("No user found with that id")),
        }
    }

    pub fn get_series(id: u32) -> Result<Series, Box<dyn Error>> {
        let mut seri = CalConnector::get_records::<Series>(&format!(
            "SELECT * FROM series where id = {id}"
        ))?;

        assert!(seri.len() == 1, "More than one series with that id! : (");

        match seri.pop() {
            Some(u) => Ok(u),
            _ => unreachable!(),
        }
    }

    pub fn get_events() -> Result<Vec<Event>, Box<dyn Error>> {
        Ok(CalConnector::get_records::<Event>(
            "SELECT id, starttime, endtime, name, caluserid, seriesid name FROM event",
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

    fn generate_random_id() -> Uuid {
        let my_uuid = Uuid::new_v4();
        my_uuid
    }
}
