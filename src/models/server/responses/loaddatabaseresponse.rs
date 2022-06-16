use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadDatabaseResponse {
    status_code: u32,
    message: String,
}

impl LoadDatabaseResponse   {
    pub fn ok() -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            LoadDatabaseResponse {
                status_code: 200,
                message: "Database loaded".to_string(),
            }
            .as_serde_string(),
        )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                LoadDatabaseResponse {
                    status_code: 500,
                    message,
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
