use crate::{
    db::calconnector::CalConnector,
    models::server::{
        requests::createeventrequest::CreateEventRequest,
        responses::{createeventresponse::CreateEventResponse, eventsresponse::EventsResponse},
    },
};
use actix_web::{get, post, web, HttpResponse};

#[post("/api/event")]
pub async fn create_event(create_event_req: web::Json<CreateEventRequest>) -> HttpResponse {
    let result = CalConnector::create_event(create_event_req.0);

    match result {
        Ok(id) => CreateEventResponse::created(id),
        Err(e) => CreateEventResponse::error(e.to_string()),
    }
}

#[get("/api/event")]
pub async fn get_events() -> HttpResponse {
    let result = CalConnector::get_events();

    match result {
        Ok(events) => EventsResponse::ok(events),
        Err(e) => EventsResponse::error(e.to_string()),
    }
}
