use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCalendarResponse {
    status_code: u32,
    message: String,
    calendar_id: Option<Uuid>,
}

impl CreateCalendarResponse {
    pub fn created(id: Uuid) -> HttpResponse {
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(
                CreateCalendarResponse {
                    status_code: 201,
                    message: "Calendar created".to_string(),
                    calendar_id: Some(id),
                }
                .as_serde_string(),
            )
    }

    pub fn bad_request(message: String) -> HttpResponse {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(
                CreateCalendarResponse {
                    calendar_id: None,
                    status_code: 400,
                    message,
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                CreateCalendarResponse {
                    status_code: 500,
                    message,
                    calendar_id: None,
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
