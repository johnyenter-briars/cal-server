use std::sync::Mutex;

use crate::{
    db::calconnector::CalConnector,
    routes::{
        adminroutes::{list_database_saves, load_database_version, save_database},
        calendarroutes::{create_calendar, delete_calendar, get_calendars_for_user},
        caluserroutes::{create_caluser, get_caluser},
        eventroutes::{create_event, delete_event, get_events, update_event},
        seriesroutes::{create_series, delete_series, get_series, get_all_series},
    },
};
use actix_web::web;
use actix_web::{middleware::Logger, App, HttpServer};

use super::apikey::ApiKey;

pub struct AppState {
    pub cal_connector: Mutex<CalConnector>, // <- Mutex is necessary to mutate safely across threads
}

pub async fn build_and_run_server(
    domain: String,
    port: u16,
    cal_connector: CalConnector,
    key_value: String,
) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        cal_connector: Mutex::new(cal_connector),
    });

    HttpServer::new(move || {
        App::new()
            .service(create_event)
            .service(get_events)
            .service(create_caluser)
            .service(update_event)
            .service(get_caluser)
            .service(create_series)
            .service(get_all_series)
            .service(get_series)
            .service(save_database)
            .service(list_database_saves)
            .service(load_database_version)
            .service(delete_event)
            .service(delete_series)
            .service(create_calendar)
            .service(get_calendars_for_user)
            .service(delete_calendar)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %U %{Content-Type}i"))
            .app_data(app_state.clone())
            .wrap(ApiKey::new(key_value.clone()))
    })
    .bind((domain, port))?
    .run()
    .await
}
