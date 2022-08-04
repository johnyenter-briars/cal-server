use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCalUserResponse {
    status_code: u32,
    message: String,
    cal_user_id: Option<Uuid>,
}

impl CreateCalUserResponse {
    pub fn created(id: Uuid) -> HttpResponse {
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(
                CreateCalUserResponse {
                    status_code: 201,
                    message: "Caluser created".to_string(),
                    cal_user_id: Some(id),
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                CreateCalUserResponse {
                    status_code: 500,
                    message,
                    cal_user_id: None,
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
