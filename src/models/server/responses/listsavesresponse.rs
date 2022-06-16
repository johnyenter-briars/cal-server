use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSavesResponse {
    saves_list: Option<Vec<String>>,
    status_code: u32,
    message: String,
}

impl ListSavesResponse  {
    pub fn ok(saves: Vec<String>) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            ListSavesResponse {
                status_code: 200,
                message: "Database saved".to_string(),
                saves_list: Some(saves),
            }
            .as_serde_string(),
        )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                ListSavesResponse {
                    status_code: 500,
                    message,
                    saves_list: None
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
