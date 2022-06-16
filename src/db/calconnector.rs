use super::{DB_FOLDER_PATH, DB_INITIAL_NAME};
use crate::models::{
    cal::{caluser::CalUser, event::Event, series::Series},
    server::requests::{
        createeventrequest::CreateEventRequest, createseriesrequest::CreateSeriesRequest,
        updateeventrequest::UpdateEventRequest,
    },
    traits::construct::ConstructableFromSql,
};

use chrono::Utc;
use rusqlite::{params, Connection};
use std::{error::Error, fs};
use uuid::Uuid;

pub struct CalConnector {
    path_to_db: String,
    delete_old_saves: bool,
}

impl CalConnector {
    pub fn new(delete_old_saves: bool) -> Self {
        let path_to_db = format!("{}{}", DB_FOLDER_PATH, DB_INITIAL_NAME);
        CalConnector {
            path_to_db,
            delete_old_saves,
        }
    }

    pub fn save_database(&self) -> Result<Uuid, Box<dyn Error>> {
        let id = CalConnector::generate_random_id();

        fs::copy(
            &self.path_to_db,
            format!("{}{}={}.db", DB_FOLDER_PATH, id, Utc::now()),
        )?;

        Ok(id)
    }

    ///TODO: this function could probably be a little more idiomatic
    pub fn list_database_saves(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut v: Vec<String> = fs::read_dir(DB_FOLDER_PATH)?
            .into_iter()
            .map(|f| f.unwrap().file_name())
            .map(|f| match f.into_string() {
                Ok(s) => s,
                Err(_) => "".to_string(),
            })
            .collect();

        v.retain(|s| s != ".gitkeep" && s != DB_INITIAL_NAME);

        Ok(v)
    }

    pub fn load_database_save(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        self.save_database()?;

        let saves = self.list_database_saves()?;

        let index_to_load = saves.iter().position(|save| {
            save.contains(&id.to_string()) 
        });

        match index_to_load {
            Some(index) => {
                let save = &saves[index];
                let database_to_set = format!("{}{}", DB_FOLDER_PATH, save);
                fs::copy(&database_to_set, &self.path_to_db)?;

                if self.delete_old_saves {
                    fs::remove_file(database_to_set)?;
                }

                Ok(())
            },
            None => Err(Box::from("No database save found with that id!")),
        }
    }

    pub fn create_caluser(
        &self,
        first_name: &str,
        last_name: &str,
        id: Option<Uuid>,
        api_key: &str,
    ) -> Result<Uuid, Box<dyn Error>> {
        let new_id = match id {
            Some(id) => id,
            None => CalConnector::generate_random_id(),
        };

        let conn = Connection::open(&self.path_to_db)?;

        conn.execute(
            "insert into caluser (id, firstname, lastname, apikey) values (?1, ?2, ?3, ?4);",
            params![new_id.to_string(), first_name, last_name, api_key],
        )?;

        Ok(new_id)
    }

    pub fn create_event(
        &self,
        event_req: CreateEventRequest,
        id: Option<Uuid>,
    ) -> Result<Uuid, Box<dyn Error>> {
        let new_id = id.unwrap_or_else(CalConnector::generate_random_id);
        let conn = Connection::open(&self.path_to_db)?;

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

    pub fn update_event(&self, event_req: UpdateEventRequest) -> Result<Uuid, Box<dyn Error>> {
        if (event_req.start_time.is_none() && event_req.end_time.is_some())
            || (event_req.start_time.is_some() && event_req.end_time.is_none())
        {
            return Err(Box::from(
                "You must construct an event with both start and end either both Some or None",
            ));
        }

        let id = event_req.id;
        let conn = Connection::open(&self.path_to_db)?;

        conn.execute(
            "UPDATE event 
                SET 
                    starttime = ?1,
                    endtime = ?2,
                    name = ?3,
                    description = ?4,
                    seriesid = ?5
                WHERE id = ?6;
                ",
            params![
                event_req.start_time.map(|t| t.timestamp()),
                event_req.end_time.map(|t| t.timestamp()),
                event_req.name,
                event_req.description,
                event_req.series_id.map(|t| t.to_string()),
                event_req.id.to_string()
            ],
        )?;

        Ok(id)
    }

    pub fn create_series(&self, series_req: CreateSeriesRequest) -> Result<Uuid, Box<dyn Error>> {
        let new_id = CalConnector::generate_random_id();
        let conn = Connection::open(&self.path_to_db)?;

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

    pub fn get_caluser(&self, uuid: Uuid) -> Result<Option<CalUser>, Box<dyn Error>> {
        let mut users = self.get_records::<CalUser>(&format!(
            "SELECT id, firstname, lastname, apikey FROM caluser where id = '{uuid}'"
        ))?;

        Ok(users.pop())
    }

    pub fn get_series(&self, id: Uuid) -> Result<Option<Series>, Box<dyn Error>> {
        let mut seri =
            self.get_records::<Series>(&format!("SELECT * FROM series where id = '{id}'"))?;

        Ok(seri.pop())
    }

    pub fn get_events(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        self.get_records::<Event>(
            "SELECT id, starttime, endtime, name, description, caluserid, seriesid name FROM event",
        )
    }

    fn get_records<T>(&self, sql: &str) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: ConstructableFromSql<T>,
    {
        let conn = Connection::open(&self.path_to_db)?;

        let mut stmt = conn.prepare(sql)?;

        let mapped_rows = stmt.query_map([], |row| Ok(T::construct(row)))?;

        let unwrapped_objects = mapped_rows
            .filter_map(|e| e.ok().map(|e2| e2.unwrap()))
            .collect();

        Ok(unwrapped_objects)
    }

    fn generate_random_id() -> Uuid {
        Uuid::new_v4()
    }
}

impl Default for CalConnector {
    fn default() -> Self {
        Self::new(false)
    }
}
