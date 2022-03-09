use crate::routes::event::{create_event, get_events};
use actix_web::{App, HttpServer};

pub async fn build_and_run_server(domain: String, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(create_event).service(get_events))
        .bind((domain, port))?
        .run()
        .await
}
