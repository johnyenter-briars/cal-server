use crate::{
    db::calconnector::CalConnector,
    models::{
        event::Event,
        responses::{eventsresponse::EventsResponse, neweventresponse::NewEventResponse},
    },
};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse};

#[post("/api/event/{event_name}")]
pub async fn create_event(event_name: web::Path<String>) -> HttpResponse {
    let new_event = Event::new(chrono::offset::Utc::now(), event_name.to_string());

    let result = CalConnector::create_event(new_event);

    match result {
        Ok(_) => HttpResponse::Created()
            .content_type(ContentType::json())
            .body(NewEventResponse::created().as_serde_string()),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(NewEventResponse::error(e.to_string()).as_serde_string()),
    }
}

#[get("/api/event")]
pub async fn get_events() -> HttpResponse {
    let result = CalConnector::get_events();

    match result {
        Ok(events) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(EventsResponse::ok(events).as_serde_string()),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(EventsResponse::error(e.to_string()).as_serde_string()),
    }
}
