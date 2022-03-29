use std::{
    fs::{self, File},
    io::{ErrorKind, Read},
};

use crate::models::server::requests::{createeventrequest::CreateEventRequest, createseriesrequest::CreateSeriesRequest};

use super::{DB_NAME, calconnector::CalConnector};
use chrono::Utc;
use rusqlite::{Connection, Result};

pub fn initiaize_db(init_test_data: bool) -> Result<(), Box<dyn std::error::Error>> {
    delete_database()?;

    let conn = Connection::open(DB_NAME)?;

    conn.execute(&get_sql_file_contents("series")?, [])?;

    conn.execute(&get_sql_file_contents("caluser")?, [])?;

    conn.execute(&get_sql_file_contents("event")?, [])?;

    drop(conn);

    if init_test_data {
        add_test_data()?;
    }

    Ok(())
}

fn get_sql_file_contents(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(format!("./db/ddl/{}.sql", file_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn delete_database() -> Result<(), Box<dyn std::error::Error>> {
    match fs::remove_file(DB_NAME) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(()),
            e => Err(Box::from(format!("{:?}", e))),
        },
    }
}

fn add_test_data() -> Result<(), Box<dyn std::error::Error>> {
    CalConnector::create_caluser("Jim", "Pankey")?;

    // An event that is 0 seconds long - not part of a series
    let event_id = CalConnector::create_event(CreateEventRequest{
        name: "first test event".to_string(),
        start_time: Some(Utc::now()),
        end_time: Some(Utc::now()),
        cal_user_id: 1,
        series_id: None,
    })?;

    //create the series
    let series_id = CalConnector::create_series(CreateSeriesRequest {
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
    CalConnector::create_event(CreateEventRequest{
        name: "first test event".to_string(),
        start_time: Some(Utc::now()),
        end_time: Some(Utc::now()),
        cal_user_id: 1,
        series_id: Some(series_id),
    })?;
    
    CalConnector::create_event(CreateEventRequest{
        name: "first test event".to_string(),
        start_time: Some(Utc::now()),
        end_time: Some(Utc::now()),
        cal_user_id: 1,
        series_id: Some(series_id),
    })?;

    Ok(())
}
