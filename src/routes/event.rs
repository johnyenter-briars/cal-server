use crate::{
    db::calconnector::CalConnector,
    models::{
        event::Event,
        responses::{eventsresponse::EventsResponse, neweventresponse::NewEventResponse},
    },
};
use actix_web::{
    get, http::header::ContentType, post, web, HttpResponse,
};

#[post("/api/event/{event_name}")]
pub async fn create_event(event_name: web::Path<String>) -> HttpResponse {
    let new_event = Event::new(chrono::offset::Utc::now(), event_name.to_string());

    let result = CalConnector::create_event(new_event);

    match result {
        Ok(_) => {
            let response_string = serde_json::to_string(&NewEventResponse::created())
                .expect("Unable to parse response object!");

            HttpResponse::Created()
                .content_type(ContentType::json())
                .body(response_string)
        }
        Err(e) => {
            let response_string = serde_json::to_string(&NewEventResponse::error(e.to_string()))
                .expect("Unable to parse response object!");

            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(response_string)
        }
    }
}

#[get("/api/event")]
pub async fn get_events() -> HttpResponse {
    let result = CalConnector::get_events();

    match result {
        Ok(events) => {
            let response_string = serde_json::to_string(&EventsResponse::ok(events))
                .expect("Unable to parse response object!");

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(response_string)
        }
        Err(e) => {
            let response_string = serde_json::to_string(&EventsResponse::error(e.to_string()))
                .expect("Unable to parse response object!");

            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(response_string)
        }
    }
}
