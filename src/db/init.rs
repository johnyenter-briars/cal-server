use std::{
    fs::{self, File},
    io::{ErrorKind, Read},
};

use crate::models::server::requests::{
    createeventrequest::CreateEventRequest, createseriesrequest::CreateSeriesRequest,
};

use super::{calconnector::CalConnector, DB_FOLDER_PATH, DB_INITIAL_NAME};
use chrono::{Duration, Utc};
use rusqlite::{Connection, Result};
use uuid::Uuid;

pub fn initiaize_db(
    init_test_data: bool,
    user_id: &str,
    api_key: &str,
    conn: &CalConnector
) -> Result<(), Box<dyn std::error::Error>> {
    delete_database()?;

    let raw_conn = Connection::open(initial_db_path())?;

    raw_conn.execute(&get_sql_file_contents("series")?, [])?;

    raw_conn.execute(&get_sql_file_contents("caluser")?, [])?;

    raw_conn.execute(&get_sql_file_contents("event")?, [])?;

    drop(raw_conn);

    if init_test_data {
        let user_id = Uuid::parse_str(user_id)?;
        add_test_data(user_id, api_key, conn)?;
    }

    Ok(())
}

fn initial_db_path() -> String {
    format!("{}{}", DB_FOLDER_PATH, DB_INITIAL_NAME)
}

fn get_sql_file_contents(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(format!("./db/ddl/{}.sql", file_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn delete_database() -> Result<(), Box<dyn std::error::Error>> {
    match fs::remove_file(initial_db_path()) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(()),
            e => Err(Box::from(format!("{:?}", e))),
        },
    }
}

fn add_test_data(user_id: Uuid, api_key: &str, conn: &CalConnector) -> Result<(), Box<dyn std::error::Error>> {
    conn.create_caluser("Jim", "Pankey", Some(user_id), api_key)?;

    // An event that is 0 seconds long - not part of a series
    conn.create_event(
        CreateEventRequest {
            name: "event1".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now()),
            end_time: Some(Utc::now() + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
        },
        None,
    )?;

    //create the series
    let series_id = conn.create_series(CreateSeriesRequest {
        repeat_every_week: 1,
        repeat_on_mon: true,
        repeat_on_tues: false,
        repeat_on_wed: true,
        repeat_on_thurs: false,
        repeat_on_fri: false,
        repeat_on_sat: false,
        repeat_on_sun: false,
        ends_on: Some(Utc::now()),
    })?;

    //create two events for it
    conn.create_event(
        CreateEventRequest {
            name: "event2".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now()),
            end_time: Some(Utc::now() + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: Some(series_id),
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "event3".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now()),
            end_time: Some(Utc::now() + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: Some(series_id),
        },
        None,
    )?;

    // An event for yesterday
    conn.create_event(
        CreateEventRequest {
            name: "yesterday".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() - Duration::days(1)),
            end_time: Some(Utc::now() - Duration::days(1) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
        },
        None,
    )?;

    // An event for romorrow
    conn.create_event(
        CreateEventRequest {
            name: "tomorrows event".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::days(1)),
            end_time: Some(Utc::now() + Duration::days(1) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
        },
        None,
    )?;

    Ok(())
}
