use crate::{
    db::calconnector::CalConnector,
    models::server::{
        requests::createeventrequest::CreateEventRequest,
        responses::{createeventresponse::CreateEventResponse, eventsresponse::EventsResponse},
    },
};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse};

#[post("/api/event")]
pub async fn create_event(create_event_req: web::Json<CreateEventRequest>) -> HttpResponse {
    let result = CalConnector::create_event(create_event_req.0);

    match result {
        Ok(id) => HttpResponse::Created()
            .content_type(ContentType::json())
            .body(CreateEventResponse::created(id).as_serde_string()),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(CreateEventResponse::error(e.to_string()).as_serde_string()),
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
