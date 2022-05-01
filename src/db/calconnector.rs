use super::DB_NAME;
use crate::models::{
    cal::{caluser::CalUser, event::Event, series::Series},
    server::requests::{
        createeventrequest::CreateEventRequest, createseriesrequest::CreateSeriesRequest,
    },
    traits::construct::ConstructableFromSql,
};
use rusqlite::{params, Connection};
use std::error::Error;
use uuid::Uuid;

pub struct CalConnector {}

impl CalConnector {
    pub fn create_caluser(
        first_name: &str,
        last_name: &str,
        id: Option<Uuid>,
        api_key: &str,
    ) -> Result<Uuid, Box<dyn Error>> {
        let new_id = match id {
            Some(id) => id,
            None => CalConnector::generate_random_id(),
        };

        let conn = Connection::open(DB_NAME)?;

        conn.execute(
            "insert into caluser (id, firstname, lastname, apikey) values (?1, ?2, ?3, ?4);",
            params![new_id.to_string(), first_name, last_name, api_key],
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
            "INSERT INTO event (id, starttime, endtime, name, description, caluserid, seriesid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                new_id.to_string(),
                event_req.start_time.map(|t| t.timestamp()),
                event_req.end_time.map(|t| t.timestamp()),
                event_req.name,
                event_req.description,
                event_req.cal_user_id.to_string(),
                event_req.series_id.map(|t| t.to_string()),
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
                series_req.ends_on.map(|t| t.timestamp()),
            ],
        )?;

        Ok(new_id)
    }

    pub fn get_caluser(uuid: Uuid) -> Result<Option<CalUser>, Box<dyn Error>> {
        let mut users = CalConnector::get_records::<CalUser>(&format!(
            "SELECT id, firstname, lastname, apikey FROM caluser where id = '{uuid}'"
        ))?;

        Ok(users.pop())
    }

    pub fn get_series(id: Uuid) -> Result<Option<Series>, Box<dyn Error>> {
        let mut seri = CalConnector::get_records::<Series>(&format!(
            "SELECT * FROM series where id = '{id}'"
        ))?;

        Ok(seri.pop())
    }

    pub fn get_events() -> Result<Vec<Event>, Box<dyn Error>> {
        CalConnector::get_records::<Event>(
            "SELECT id, starttime, endtime, name, description, caluserid, seriesid name FROM event",
        )
    }

    fn get_records<T>(sql: &str) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: ConstructableFromSql<T>,
    {
        let conn = Connection::open(DB_NAME)?;

        let mut stmt = conn.prepare(sql)?;

        let mapped_rows = stmt.query_map([], |row| Ok(T::construct(row)))?;

        let unwrapped_objects = mapped_rows
            .filter_map(|e| match e.ok() {
                Some(e2) => Some(e2.unwrap()),
                None => None,
            })
            .collect();

        Ok(unwrapped_objects)
    }

    fn generate_random_id() -> Uuid {
        Uuid::new_v4()
    }
}
