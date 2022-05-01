use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSeriesResponse {
    status_code: u32,
    message: String,
    series_id: Option<Uuid>,
}

impl CreateSeriesResponse {
    pub fn created(series_id: Uuid) -> HttpResponse {
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(
                CreateSeriesResponse {
                    status_code: 201,
                    message: "Series created".to_string(),
                    series_id: Some(series_id),
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                CreateSeriesResponse {
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
