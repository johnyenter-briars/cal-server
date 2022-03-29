use std::{
    fs::{self, File},
    io::{ErrorKind, Read},
};

use crate::models::server::requests::createeventrequest::CreateEventRequest;

use super::{DB_NAME, calconnector::CalConnector};
use chrono::Utc;
use rusqlite::{Connection, Result};

pub fn initiaize_db(init_test_data: bool) -> Result<(), Box<dyn std::error::Error>> {
    delete_database()?;

    let conn = Connection::open(DB_NAME)?;

    let create_caluser = get_sql_file_contents("create_caluser")?;
    conn.execute(&create_caluser, [])?;

    let create_event = get_sql_file_contents("create_event")?;
    conn.execute(&create_event, [])?;

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

    CalConnector::create_event(CreateEventRequest{
        name: "first test event".to_string(),
        time: Utc::now(),
        cal_user_id: 1,
    })?;

    CalConnector::create_event(CreateEventRequest{
        name: "second test event".to_string(),
        time: Utc::now(),
        cal_user_id: 1,
    })?;

    CalConnector::create_event(CreateEventRequest{
        name: "third test event".to_string(),
        time: Utc::now(),
        cal_user_id: 1,
    })?;

    Ok(())
}
