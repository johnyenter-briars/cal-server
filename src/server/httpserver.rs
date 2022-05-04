use crate::routes::{
    caluserroutes::{create_caluser, get_caluser},
    eventroutes::{create_event, get_events, update_event},
    seriesroutes::{create_series, get_series},
};
use actix_web::{
    middleware::Logger,
    App, HttpServer,
};

use super::apikey::ApiKey;

pub async fn build_and_run_server(
    domain: String,
    port: u16,
    key_value: String,
) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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
            .wrap(ApiKey::new(key_value.clone()))
    })
    .bind((domain, port))?
    .run()
    .await
}