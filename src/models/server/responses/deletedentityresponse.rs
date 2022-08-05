use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedEntityResponse {
    id: Option<Uuid>,
    status_code: u32,
    message: String,
}

impl DeletedEntityResponse {
    pub fn ok(id: Uuid) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            DeletedEntityResponse {
                status_code: 200,
                message: "Entity deleted".to_string(),
                id: Some(id),
            }
            .as_serde_string(),
        )
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(
                DeletedEntityResponse {
                    status_code: 400,
                    message: "No entity found with that id".to_string(),
                    id: None,
                }
                .as_serde_string(),
            )
    }

    pub fn bad_request(message: String) -> HttpResponse {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(
                DeletedEntityResponse {
                    status_code: 400,
                    message,
                    id: None,
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                DeletedEntityResponse {
                    status_code: 400,
                    message,
                    id: None,
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
