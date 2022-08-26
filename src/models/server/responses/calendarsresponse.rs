use crate::models::cal::calendar::Calendar;
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarsResponse {
    calendars: Vec<Calendar>,
    status_code: u32,
    message: String,
}

impl CalendarsResponse {
    pub fn ok(calendars: Vec<Calendar>) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            CalendarsResponse {
                status_code: 200,
                message: "Calendars found".to_string(),
                calendars,
            }
            .as_serde_string(),
        )
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(
                CalendarsResponse {
                    status_code: 400,
                    message: "No calendars found for user".to_string(),
                    calendars: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                CalendarsResponse {
                    status_code: 500,
                    message,
                    calendars: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
