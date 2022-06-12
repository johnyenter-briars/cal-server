use std::sync::Mutex;

use crate::{routes::{
    caluserroutes::{create_caluser, get_caluser},
    eventroutes::{create_event, get_events, update_event},
    seriesroutes::{create_series, get_series},
}, db::calconnector::CalConnector};
use actix_web::web;
use actix_web::{middleware::Logger, App, HttpServer};

pub struct AppState {
    pub cal_connector: Mutex<CalConnector>, // <- Mutex is necessary to mutate safely across threads
}

pub async fn build_and_run_server(domain: String, port: u16, cal_connector: CalConnector) -> std::io::Result<()> {
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
            .service(get_series)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %U %{Content-Type}i"))
            .app_data(app_state.clone())
    })
    .bind((domain, port))?
    .run()
    .await
}
