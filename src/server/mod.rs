use crate::routes::{eventroutes::{create_event, get_events}, caluserroutes::{create_caluser, get_caluser}};
use actix_web::{App, HttpServer, middleware::Logger};

pub async fn build_and_run_server(domain: String, port: u16) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| App::new()
            .service(create_event)
            .service(get_events)
            .service(create_caluser)
            .service(get_caluser)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %U %{Content-Type}i"))
        )
        .bind((domain, port))?
        .run()
        .await
}
