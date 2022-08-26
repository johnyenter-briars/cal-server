use crate::models::cal::series::Series;
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllSeriesResponse {
    series: Vec<Series>,
    status_code: u32,
    message: String,
}

impl AllSeriesResponse {
    pub fn ok(series: Vec<Series>) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            AllSeriesResponse {
                status_code: 200,
                message: "All series found".to_string(),
                series,
            }
            .as_serde_string(),
        )
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(
                AllSeriesResponse {
                    status_code: 400,
                    message: "No series found with that id".to_string(),
                    series: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn bad_request(message: String) -> HttpResponse {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(
                AllSeriesResponse {
                    status_code: 400,
                    message,
                    series: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                AllSeriesResponse {
                    status_code: 500,
                    message,
                    series: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
