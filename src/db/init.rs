use std::{
    fs::{self, File},
    io::{ErrorKind, Read},
};

use crate::{
    models::server::requests::{
        createcalendarrequest::CreateCalendarRequest, createeventrequest::CreateEventRequest,
        createnotificationrequest::CreateNotificationRequest,
        createseriesrequest::CreateSeriesRequest,
        createsharedcalendarrequest::CreateSharedCalendarRequest,
    },
    CalResult,
};

use super::{calconnector::CalConnector, DB_FOLDER_PATH, DB_INITIAL_NAME};
use chrono::{Duration, Utc};
use rusqlite::Connection;
use uuid::Uuid;

pub fn initiaize_db(
    init_test_data: bool,
    user_id: &str,
    api_key: &str,
    conn: &CalConnector,
) -> CalResult<()> {
    delete_database()?;

    let raw_conn = Connection::open(initial_db_path())?;

    raw_conn.execute(&get_sql_file_contents("series")?, [])?;

    raw_conn.execute(&get_sql_file_contents("caluser")?, [])?;

    raw_conn.execute(&get_sql_file_contents("event")?, [])?;

    raw_conn.execute(&get_sql_file_contents("calendar")?, [])?;

    raw_conn.execute(&get_sql_file_contents("sharedcalendar")?, [])?;

    raw_conn.execute(&get_sql_file_contents("notification")?, [])?;

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

fn get_sql_file_contents(file_name: &str) -> CalResult<String> {
    let mut file = File::open(format!("./db/ddl/{}.sql", file_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn delete_database() -> CalResult<()> {
    match fs::remove_file(initial_db_path()) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(()),
            e => Err(Box::from(format!("{:?}", e))),
        },
    }
}

fn add_test_data(user_id: Uuid, api_key: &str, conn: &CalConnector) -> CalResult<()> {
    conn.create_caluser("Jim", "Pankey", Some(user_id), api_key)?;
    let second_user_id = Uuid::parse_str("b7791e67-bff0-465b-aef5-de3d3d13033e")?;
    conn.create_caluser("Second", "User", Some(second_user_id), api_key)?;

    let bday_calendar_id = conn.create_calendar(
        CreateCalendarRequest {
            name: "Birthdays".to_string(),
            description: None,
            cal_user_id: user_id,
            color: "Blue".to_string(),
        },
        Some(Uuid::parse_str("aebb3df3-d1fa-4f21-af2f-a98d0774f3ac")?),
    )?;

    let second_user_bday_calendar_id = conn.create_calendar(
        CreateCalendarRequest {
            name: "Birthdays - second user".to_string(),
            description: None,
            cal_user_id: second_user_id,
            color: "Blue".to_string(),
        },
        None,
    )?;

    let work_calendar_id = conn.create_calendar(
        CreateCalendarRequest {
            name: "work".to_string(),
            description: None,
            cal_user_id: user_id,
            color: "Red".to_string(),
        },
        None,
    )?;

    let second_user_work_calendar_id = conn.create_calendar(
        CreateCalendarRequest {
            name: "work - second user".to_string(),
            description: None,
            cal_user_id: second_user_id,
            color: "Red".to_string(),
        },
        None,
    )?;

    let shared_calendar_id = conn.create_shared_calendar(
        CreateSharedCalendarRequest {
            name: "shared calendar".to_string(),
            description: None,
            owner_cal_user_id: user_id,
            cal_users: vec![user_id, second_user_id],
            color: "Red".to_string(),
        },
        Some(Uuid::parse_str("74dddbb0-1a3b-4426-94b1-c4d951e59a23")?),
    )?;

    for e in 0..2 {
        conn.create_event(
            CreateEventRequest {
                name: format!("event: {}", e).to_string(),
                description: Some("some description here".to_string()),
                start_time: Some(Utc::now() + Duration::minutes(7)),
                end_time: Some(Utc::now() + Duration::hours(1)),
                cal_user_id: user_id,
                series_id: None,
                calendar_id: work_calendar_id,
                color: Some("Purple".to_string()),
                num_times_notified: 0,
                should_notify: true,
            },
            None,
        )?;

        conn.create_event(
            CreateEventRequest {
                name: format!("event: {}", e).to_string(),
                description: Some("some description here".to_string()),
                start_time: Some(Utc::now() + Duration::minutes(7)),
                end_time: Some(Utc::now() + Duration::hours(1)),
                cal_user_id: user_id,
                series_id: None,
                calendar_id: second_user_work_calendar_id,
                color: Some("Purple".to_string()),
                num_times_notified: 0,
                should_notify: true,
            },
            None,
        )?;

        conn.create_event(
            CreateEventRequest {
                name: format!("shared event: {}", e).to_string(),
                description: Some("some description here".to_string()),
                start_time: Some(Utc::now() + Duration::minutes(7)),
                end_time: Some(Utc::now() + Duration::hours(1)),
                cal_user_id: user_id,
                series_id: None,
                calendar_id: shared_calendar_id,
                color: Some("Purple".to_string()),
                num_times_notified: 0,
                should_notify: true,
            },
            None,
        )?;
    }
    conn.create_event(
        CreateEventRequest {
            name: "This event should notify".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(7)),
            end_time: Some(Utc::now() + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: work_calendar_id,
            color: Some("Purple".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "this should be in ebuary".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::days(20)),
            end_time: Some(Utc::now() + Duration::days(20) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: work_calendar_id,
            color: Some("Purple".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "work event 2".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(3)),
            end_time: Some(Utc::now() + Duration::days(2) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: work_calendar_id,
            color: Some("Pink".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "work event 3".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(4)),
            end_time: Some(Utc::now() + Duration::days(2) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: work_calendar_id,
            color: Some("Pink".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "work event 4".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(5)),
            end_time: Some(Utc::now() + Duration::days(2) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: work_calendar_id,
            color: Some("Yellow".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "work event 5".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(17)),
            end_time: Some(Utc::now() + Duration::days(2) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: work_calendar_id,
            color: Some("Yellow".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    // An event that is 0 seconds long - not part of a series
    conn.create_event(
        CreateEventRequest {
            name: "event idk".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now()),
            end_time: Some(Utc::now() + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: bday_calendar_id,
            color: Some("Pink".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "event idk".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now()),
            end_time: Some(Utc::now() + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: second_user_bday_calendar_id,
            color: Some("Pink".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    //create the series
    let series_id = conn.create_series(
        CreateSeriesRequest {
            name: "series test".to_string(),
            description: Some("i literally dk".to_string()),
            repeat_every_week: 1,
            repeat_on_mon: true,
            repeat_on_tues: false,
            repeat_on_wed: true,
            repeat_on_thurs: false,
            repeat_on_fri: false,
            repeat_on_sat: false,
            repeat_on_sun: false,
            starts_on: Utc::now(),
            ends_on: Utc::now() + Duration::days(1),
            event_start_time: chrono::Duration::seconds(1000),
            event_end_time: chrono::Duration::seconds(1000),
            cal_user_id: user_id,
            calendar_id: bday_calendar_id,
            color: "Blue".to_string(),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    //create two events for it
    conn.create_event(
        CreateEventRequest {
            name: "this is part of a series".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(30)),
            end_time: Some(Utc::now() + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: Some(series_id),
            calendar_id: bday_calendar_id,
            color: Some("Green".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "this is part of a series part 2".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(30)),
            end_time: Some(Utc::now() + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: Some(series_id),
            calendar_id: bday_calendar_id,
            color: Some("Purple".to_string()),
            num_times_notified: 0,
            should_notify: true,
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
            calendar_id: bday_calendar_id,
            color: Some("Orange".to_string()),
            num_times_notified: 0,
            should_notify: true,
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
            calendar_id: bday_calendar_id,
            color: Some("Green".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "this should be in match".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::weeks(5)),
            end_time: Some(Utc::now() + Duration::weeks(5) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: bday_calendar_id,
            color: Some("Pink".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    conn.create_event(
        CreateEventRequest {
            name: "this should be in the future".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::weeks(25)),
            end_time: Some(Utc::now() + Duration::weeks(25) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: bday_calendar_id,
            color: Some("Purple".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    let notificaion_test_event_1 = conn.create_event(
        CreateEventRequest {
            name: "notification test event".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(10)),
            end_time: Some(Utc::now() + Duration::minutes(10) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: work_calendar_id,
            color: Some("Green".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    let _ = conn.create_notification(
        CreateNotificationRequest {
            event_id: notificaion_test_event_1,
            cal_user_id: user_id,
        },
        None,
    );

    let _ = conn.create_event(
        CreateEventRequest {
            name: "shared_notification_test_event_1".to_string(),
            description: Some("some description here".to_string()),
            start_time: Some(Utc::now() + Duration::minutes(10)),
            end_time: Some(Utc::now() + Duration::minutes(10) + Duration::hours(1)),
            cal_user_id: user_id,
            series_id: None,
            calendar_id: shared_calendar_id,
            color: Some("Green".to_string()),
            num_times_notified: 0,
            should_notify: true,
        },
        None,
    )?;

    Ok(())
}
