use crate::routes::event::{create_event, get_events};
use actix_web::{App, HttpServer, middleware::Logger};

pub async fn build_and_run_server(domain: String, port: u16) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| App::new()
            .service(create_event)
            .service(get_events)
            .wrap(Logger::new("%a %{User-Agent}i %r %s"))
        )
        .bind((domain, port))?
        .run()
        .await
}
