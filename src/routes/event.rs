use actix_web::{post, web, App, HttpServer, Responder};

#[post("/api/event/{event_name}")]
pub async fn create_event(event_name: web::Path<String>) -> impl Responder {
    format!("Hello {event_name}!")
}