use std::{
    fs::{self, File},
    io::{ErrorKind, Read},
};

use super::DB_NAME;
use rusqlite::{Connection, Result};

pub fn initiaize_db(init_test_data: bool) -> Result<(), Box<dyn std::error::Error>> {
    delete_database()?;

    let conn = Connection::open(DB_NAME)?;

    let create_caluser = get_sql_file_contents("create_caluser")?;
    conn.execute(&create_caluser, [])?;

    let create_event = get_sql_file_contents("create_event")?;
    conn.execute(&create_event, [])?;

    if init_test_data {
        add_test_data(&conn)?;
    }

    match conn.close() {
        Ok(_) => Ok(()),
        Err((_, err)) => Err(Box::new(err)),
    }
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

fn add_test_data(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "insert into caluser (first_name, second_name) values ('Jim', 'Pankey');",
        [],
    )?;
    conn.execute(
        "insert into event (time, name, caluserid) values (100, 'test event', 1);",
        [],
    )?;

    Ok(())
}
