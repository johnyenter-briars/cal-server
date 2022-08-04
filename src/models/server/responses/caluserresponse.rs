use crate::models::cal::caluser::CalUser;
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalUserResponse {
    user: Option<CalUser>,
    status_code: u32,
    message: String,
}

impl CalUserResponse {
    pub fn ok(user: CalUser) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            CalUserResponse {
                status_code: 200,
                message: "CalUser found".to_string(),
                user: Some(user),
            }
            .as_serde_string(),
        )
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(
                CalUserResponse {
                    status_code: 400,
                    message: "No user found with that id".to_string(),
                    user: None,
                }
                .as_serde_string(),
            )
    }

    pub fn bad_request(message: String) -> HttpResponse {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(
                CalUserResponse {
                    status_code: 400,
                    message,
                    user: None,
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                CalUserResponse {
                    status_code: 400,
                    message,
                    user: None,
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
