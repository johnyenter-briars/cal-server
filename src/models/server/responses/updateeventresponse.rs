use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

use crate::models::traits::validate::Validatable;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEventResponse {
    status_code: u32,
    message: String,
    event_id: Option<Uuid>,
}

impl UpdateEventResponse {
    pub fn created(id: Uuid) -> HttpResponse {
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(
                UpdateEventResponse {
                    status_code: 201,
                    message: "Event created".to_string(),
                    event_id: Some(id),
                }
                .as_serde_string(),
            )
    }

    pub fn updated(id: Uuid) -> HttpResponse {
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(
                UpdateEventResponse {
                    status_code: 200,
                    message: "Event updated".to_string(),
                    event_id: Some(id),
                }
                .as_serde_string(),
            )
    }

    pub fn bad_request(message: String) -> HttpResponse {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(
                UpdateEventResponse {
                    event_id: None,
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
                UpdateEventResponse {
                    status_code: 500,
                    message,
                    event_id: None,
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
