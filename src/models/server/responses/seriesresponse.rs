use crate::models::cal::series::Series;
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesResponse {
    series: Option<Series>,
    status_code: u32,
    message: String,
}

impl SeriesResponse {
    pub fn ok(series: Series) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            SeriesResponse {
                status_code: 200,
                message: "Series found".to_string(),
                series: Some(series),
            }
            .as_serde_string(),
        )
    }

    pub fn bad_request(message: String) -> HttpResponse {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(
                SeriesResponse {
                    status_code: 400,
                    message,
                    series: None,
                }
                .as_serde_string(),
            )
    }


    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                SeriesResponse {
                    status_code: 500,
                    message,
                    series: None,
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
