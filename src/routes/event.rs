use crate::{db::calconnector::CalConnector, models::{event::Event, error::{Error, self}, self, responses::eventsresponse::EventsResponse}};
use actix_web::{post, web, App, HttpServer, Responder, get, Result};

#[post("/api/event/{event_name}")]
pub async fn create_event(event_name: web::Path<String>) -> impl Responder {
    let new_event = Event::new(chrono::offset::Utc::now(), event_name.to_string());

    let result = CalConnector::create_event(new_event);

    match result {
        Ok(r) => format!("Success? {:?}", r),
        Err(e) => format!("Something went wrong! {:?}", e),
    }
}

#[get("/api/event")]
pub async fn get_events() -> Result<impl Responder> {
    let result = CalConnector::get_events();

    match result {
        Ok(r) => Ok(web::Json(EventsResponse{
            events: r
        })),
        Err(e) => panic!("uhhhh....."),
    }    
}
