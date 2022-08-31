
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSeriesResponse {
    status_code: u32,
    message: String,
    series_id: Option<Uuid>,
}

impl UpdateSeriesResponse {
    pub fn created(id: Uuid) -> HttpResponse {
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(
                UpdateSeriesResponse {
                    status_code: 201,
                    message: "Event created".to_string(),
                    series_id: Some(id),
                }
                .as_serde_string(),
            )
    }

    pub fn updated(id: Uuid) -> HttpResponse {
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(
                UpdateSeriesResponse {
                    status_code: 200,
                    message: "Event updated".to_string(),
                    series_id: Some(id),
                }
                .as_serde_string(),
            )
    }

    pub fn bad_request(message: String) -> HttpResponse {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(
                UpdateSeriesResponse {
                    series_id: None,
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
                UpdateSeriesResponse {
                    status_code: 500,
                    message,
                    series_id: None,
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
