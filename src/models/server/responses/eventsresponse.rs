use crate::models::cal::event::Event;
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsResponse {
    events: Vec<Event>,
    status_code: u32,
    message: String,
}

impl EventsResponse {
    pub fn ok(events: Vec<Event>) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            EventsResponse {
                status_code: 200,
                message: "Events found".to_string(),
                events,
            }
            .as_serde_string(),
        )
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(
                EventsResponse {
                    status_code: 400,
                    message: "No events found for user".to_string(),
                    events: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                EventsResponse {
                    status_code: 500,
                    message,
                    events: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
