
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDatabaseResult {
    id: Option<Uuid>,
    status_code: u32,
    message: String,
}

impl SaveDatabaseResult {
    pub fn ok(id: Uuid) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            SaveDatabaseResult {
                status_code: 200,
                message: "Database saved".to_string(),
                id: Some(id),
            }
            .as_serde_string(),
        )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                SaveDatabaseResult {
                    status_code: 500,
                    message,
                    id: None
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
