use crate::routes::{
    caluserroutes::{create_caluser, get_caluser},
    eventroutes::{create_event, get_events},
    seriesroutes::{create_series, get_series},
};
use actix_web::{middleware::Logger, App, HttpServer};

pub async fn build_and_run_server(domain: String, port: u16) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(create_event)
            .service(get_events)
            .service(create_caluser)
            .service(get_caluser)
            .service(create_series)
            .service(get_series)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %U %{Content-Type}i"))
    })
    .bind((domain, port))?
    .run()
    .await
}
