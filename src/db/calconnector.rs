use super::{DB_FOLDER_PATH, DB_INITIAL_NAME};
use crate::{
    models::{
        cal::{
            calendar::Calendar, caluser::CalUser, event::Event, notification::Notification,
            series::Series, sharedcalendar::SharedCalendar,
        },
        server::requests::{
            createcalendarrequest::CreateCalendarRequest, createeventrequest::CreateEventRequest,
            createnotificationrequest::CreateNotificationRequest,
            createseriesrequest::CreateSeriesRequest,
            createsharedcalendarrequest::CreateSharedCalendarRequest,
            updateeventrequest::UpdateEventRequest,
        },
        traits::construct::ConstructableFromSql,
    },
    CalResult,
};

use chrono::{DateTime, Datelike, Utc};
use rusqlite::{params, Connection};
use std::fs;
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

    pub fn save_database(&self) -> CalResult<Uuid> {
        let id = CalConnector::generate_random_id();

        fs::copy(
            &self.path_to_db,
            format!("{}{}={}.db", DB_FOLDER_PATH, id, Utc::now()),
        )?;

        Ok(id)
    }

    pub fn list_database_saves(&self) -> CalResult<Vec<String>> {
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

    pub fn load_database_save(&self, id: Uuid) -> CalResult<()> {
        self.save_database()?;

        let saves = self.list_database_saves()?;

        let index_to_load = saves.iter().position(|save| save.contains(&id.to_string()));

        match index_to_load {
            Some(index) => {
                let save = &saves[index];
                let database_to_set = format!("{}{}", DB_FOLDER_PATH, save);
                fs::copy(&database_to_set, &self.path_to_db)?;

                if self.delete_old_saves {
                    fs::remove_file(database_to_set)?;
                }

                Ok(())
            }
            None => Err(Box::from("No database save found with that id!")),
        }
    }

    pub fn create_caluser(
        &self,
        first_name: &str,
        last_name: &str,
        id: Option<Uuid>,
        api_key: &str,
    ) -> CalResult<Uuid> {
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

    pub fn create_event(&self, event_req: CreateEventRequest, id: Option<Uuid>) -> CalResult<Uuid> {
        let color = match event_req.series_id {
            Some(id) => self.get_series(id)?.unwrap().color,
            None => event_req.color.unwrap_or_else(|| "red".to_string()),
        };

        let new_id = id.unwrap_or_else(CalConnector::generate_random_id);
        let conn = Connection::open(&self.path_to_db)?;

        conn.execute(
            "INSERT INTO event (id, starttime, endtime, name, description, caluserid, seriesid, calendarid, color, numtimesnotified, shouldnotify) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                new_id.to_string(),
                event_req.start_time.map(|t| t.timestamp()),
                event_req.end_time.map(|t| t.timestamp()),
                event_req.name,
                event_req.description,
                event_req.cal_user_id.to_string(),
                event_req.series_id.map(|t| t.to_string()),
                event_req.calendar_id.to_string(),
                color,
                event_req.num_times_notified,
                event_req.should_notify
            ],
        )?;

        Ok(new_id)
    }

    pub fn create_calendar(
        &self,
        calendar_req: CreateCalendarRequest,
        id: Option<Uuid>,
    ) -> CalResult<Uuid> {
        let new_id = id.unwrap_or_else(CalConnector::generate_random_id);
        let conn = Connection::open(&self.path_to_db)?;

        conn.execute(
            "INSERT INTO calendar (id, name, description, caluserid, color) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                new_id.to_string(),
                calendar_req.name,
                calendar_req.description,
                calendar_req.cal_user_id.to_string(),
                calendar_req.color.to_string(),
            ],
        )?;

        Ok(new_id)
    }

    pub fn create_shared_calendar(
        &self,
        calendar_req: CreateSharedCalendarRequest,
        id: Option<Uuid>,
    ) -> CalResult<Uuid> {
        let conn = Connection::open(&self.path_to_db)?;

        let base_calendar_id = id.unwrap_or_else(CalConnector::generate_random_id);

        conn.execute(
            "INSERT INTO calendar (id, name, description, caluserid, color) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                base_calendar_id.to_string(),
                calendar_req.name,
                calendar_req.description,
                calendar_req.owner_cal_user_id.to_string(),
                calendar_req.color.to_string(),
            ],
        )?;

        for cal_user_id in calendar_req.cal_users {
            let shared_calendar_id = CalConnector::generate_random_id();
            conn.execute(
                "INSERT INTO sharedcalendar (id, calendarid, ownercaluserid, caluserid) VALUES (?1, ?2, ?3, ?4)",
                params![
                    shared_calendar_id.to_string(),
                    base_calendar_id.to_string(),
                    calendar_req.owner_cal_user_id.to_string(),
                    cal_user_id.to_string(),
                ],
            )?;
        }

        Ok(base_calendar_id)
    }

    pub fn create_notification(
        &self,
        notification_req: CreateNotificationRequest,
        id: Option<Uuid>,
    ) -> CalResult<Uuid> {
        let conn = Connection::open(&self.path_to_db)?;

        let notification_id = id.unwrap_or_else(CalConnector::generate_random_id);

        conn.execute(
            "INSERT INTO notification (id, eventid, caluserid) VALUES (?1, ?2, ?3)",
            params![
                notification_id.to_string(),
                notification_req.event_id.to_string(),
                notification_req.cal_user_id.to_string(),
            ],
        )?;

        Ok(notification_id)
    }

    pub fn get_upcomming_events(&self, cal_user_id: Uuid) -> CalResult<Vec<Event>> {
        let now: DateTime<Utc> = Utc::now();

        let events = self.get_events_month_year(now.year(), now.month())?;

        let calendars_for_user = self
            .get_calendars_for_user(cal_user_id)?
            .into_iter()
            .map(|c| c.id)
            .collect::<Vec<Uuid>>();

        let mut return_events: Vec<Event> = vec![];

        let events_in_calendars = events
            .into_iter()
            .filter(|e| calendars_for_user.contains(&e.calendar_id));

        for e in events_in_calendars {
            // Only consider events happening today
            if let Some(start) = e.start_time {
                if now.date_naive() != start.date_naive() || !e.should_notify {
                    continue;
                }

                let num_events = self.get_notifications_for_event(e.id, cal_user_id)?;

                //TODO: magic number
                if num_events.len() >= 3 {
                    continue;
                }

                let span = start - now;

                // future-only checks
                let minutes = span.num_minutes();

                // 16–30 minute window
                if (16..=30).contains(&minutes) {
                    return_events.push(e.clone());
                }

                // 0–15 minute window
                if (0..=15).contains(&minutes) {
                    return_events.push(e.clone());
                }
            }
        }

        Ok(return_events)
    }

    pub fn update_event(&self, event_req: UpdateEventRequest) -> CalResult<Uuid> {
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
                    seriesid = ?5,
                    color = ?6,
                    numtimesnotified = ?7,
                    shouldnotify = ?8
                WHERE id = ?9;
                ",
            params![
                event_req.start_time.map(|t| t.timestamp()),
                event_req.end_time.map(|t| t.timestamp()),
                event_req.name,
                event_req.description,
                event_req.series_id.map(|t| t.to_string()),
                event_req.color,
                event_req.num_times_notified,
                event_req.should_notify,
                event_req.id.to_string()
            ],
        )?;

        Ok(id)
    }

    pub fn create_series(
        &self,
        series_req: CreateSeriesRequest,
        id: Option<Uuid>,
    ) -> CalResult<Uuid> {
        let new_id = id.unwrap_or_else(CalConnector::generate_random_id);
        // let new_id = CalConnector::generate_random_id();
        let conn = Connection::open(&self.path_to_db)?;

        conn.execute(
            "INSERT INTO series (
                id,
                name,
                description,
                repeateveryweek,
                repeateonmon,
                repeateontues,
                repeateonwed,
                repeateonthurs,
                repeateonfri,
                repeateonsat,
                repeateonsun,
                startson,
                endson,
                eventstarttime,
                eventendtime,
                caluserid, 
                calendarid,
                color,
                numtimesnotified,
                shouldnotify
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
            params![
                new_id.to_string(),
                series_req.name,
                series_req.description,
                series_req.repeat_every_week,
                series_req.repeat_on_mon,
                series_req.repeat_on_tues,
                series_req.repeat_on_wed,
                series_req.repeat_on_thurs,
                series_req.repeat_on_fri,
                series_req.repeat_on_sat,
                series_req.repeat_on_sun,
                series_req.starts_on.timestamp(),
                series_req.ends_on.timestamp(),
                series_req.event_start_time.num_seconds(),
                series_req.event_end_time.num_seconds(),
                series_req.cal_user_id.to_string(),
                series_req.calendar_id.to_string(),
                series_req.color.to_string(),
                series_req.num_times_notified,
                series_req.should_notify,
            ],
        )?;

        Ok(new_id)
    }

    pub fn get_caluser(&self, uuid: Uuid) -> CalResult<Option<CalUser>> {
        let mut users =
            self.get_records::<CalUser>(&format!("SELECT * FROM caluser where id = '{uuid}'"))?;

        Ok(users.pop())
    }

    pub fn get_series(&self, id: Uuid) -> CalResult<Option<Series>> {
        let mut seri =
            self.get_records::<Series>(&format!("SELECT * FROM series where id = '{id}'"))?;

        Ok(seri.pop())
    }

    pub fn get_all_series(&self) -> CalResult<Vec<Series>> {
        self.get_records::<Series>("SELECT * FROM series")
    }

    pub fn get_all_notifications(&self) -> CalResult<Vec<Notification>> {
        self.get_records::<Notification>("SELECT * FROM notification")
    }

    pub fn get_notifications_for_event(
        &self,
        event_id: Uuid,
        cal_user_id: Uuid,
    ) -> CalResult<Vec<Notification>> {
        self.get_records::<Notification>(&format!("SELECT * FROM notification where eventid = '{event_id}' and caluserid = '{cal_user_id}'"))
    }

    pub fn get_calendars_for_user(&self, cal_user_id: Uuid) -> CalResult<Vec<Calendar>> {
        let calendars = self.get_records::<Calendar>("SELECT * FROM calendar")?;
        let shared_calendars =
            self.get_records::<SharedCalendar>("SELECT * FROM sharedcalendar")?;

        let shared_calendar_ids_the_user_is_in = shared_calendars
            .into_iter()
            .filter(|c| c.cal_user_id == cal_user_id)
            .map(|c| c.calendar_id)
            .collect::<Vec<Uuid>>();

        let calendars_for_user = calendars
            .into_iter()
            .filter(|c| {
                c.cal_user_id == cal_user_id || shared_calendar_ids_the_user_is_in.contains(&c.id)
            })
            .collect::<Vec<Calendar>>();

        Ok(calendars_for_user)
    }

    pub fn delete_calendar(&self, id: Uuid) -> CalResult<Option<Uuid>> {
        let series_in_calendar = self
            .get_all_series()?
            .into_iter()
            .filter(|s| s.calendar_id == id)
            .collect::<Vec<Series>>();

        let events_in_calendar = self
            .get_events()?
            .into_iter()
            .filter(|e| e.calendar_id == id)
            .collect::<Vec<Event>>();

        for s in series_in_calendar {
            let _ = self.delete_series(s.id)?;
        }

        for e in events_in_calendar {
            let _ = self.delete_entity(e.id, "event")?;
        }

        self.delete_entity(id, "calendar")
    }

    pub fn delete_series(&self, id: Uuid) -> CalResult<Option<Uuid>> {
        let sub_events = self
            .get_events()?
            .into_iter()
            .filter(|e| match e.series_id {
                Some(e_id) => e_id == id,
                None => false,
            })
            .collect::<Vec<Event>>();

        for sub_event in sub_events {
            //I really hope nothing bad goes wrong here : /
            let _ = self.delete_entity(sub_event.id, "event")?;
        }

        self.delete_entity(id, "series")
    }

    pub fn delete_entity(&self, id: Uuid, entity: &str) -> CalResult<Option<Uuid>> {
        let conn = Connection::open(&self.path_to_db)?;

        let mut stmt = conn.prepare(&format!("DELETE FROM {entity} WHERE id = '{id}'"))?;

        let number_rows_updated = stmt.execute(params![])?;

        if number_rows_updated == 0 {
            return Ok(None);
        }

        Ok(Some(id))
    }

    pub fn get_events(&self) -> CalResult<Vec<Event>> {
        self.get_records::<Event>("SELECT * FROM event")
    }

    pub fn get_events_via_name(&self, name: String) -> CalResult<Vec<Event>> {
        self.get_records::<Event>(&format!("SELECT * FROM event where name LIKE '%{}%'", name))
    }

    pub fn get_events_month_year(&self, year: i32, month: u32) -> CalResult<Vec<Event>> {
        let events = self.get_records::<Event>("SELECT * FROM event")?;

        let filtered = events
            .into_iter()
            .filter(|e| match e.start_time {
                Some(d) => d.month() == month && d.year() == year,
                None => false,
            })
            .collect::<Vec<Event>>();

        Ok(filtered)
    }

    fn get_records<T>(&self, sql: &str) -> CalResult<Vec<T>>
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
