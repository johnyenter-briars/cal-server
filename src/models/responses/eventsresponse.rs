use crate::models::event::Event;
use serde::Serialize;

#[derive(Serialize)]
pub struct EventsResponse { 
    events: Vec<Event>,
    status_code: u32,
    message: String,
}

impl EventsResponse {
    pub fn ok(events: Vec<Event>) -> Self {
        EventsResponse{status_code: 200, message: "Events found".to_string(), events}
    }

    pub fn error(message: String) -> Self {
        EventsResponse{status_code: 500, message, events: vec![]}
    }
}