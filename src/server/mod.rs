use actix_web::{HttpServer, App};
use crate::routes::event::create_event;

pub async fn build_and_run_server(domain: String, port: u16) -> std::io::Result<()>  {
    HttpServer::new(|| App::new().service(create_event))
        .bind((domain, port))?
        .run()
        .await
}
