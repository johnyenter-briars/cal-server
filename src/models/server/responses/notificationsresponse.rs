use crate::models::cal::{calendar::Calendar, notification::Notification};
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationsResponse {
    notifications: Vec<Notification>,
    status_code: u32,
    message: String,
}

impl NotificationsResponse {
    pub fn ok(notifications: Vec<Notification>) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            NotificationsResponse {
                status_code: 200,
                message: "Notifications found".to_string(),
                notifications,
            }
            .as_serde_string(),
        )
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(
                NotificationsResponse {
                    status_code: 400,
                    message: "No Notifications found for user".to_string(),
                    notifications: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                NotificationsResponse {
                    status_code: 500,
                    message,
                    notifications: vec![],
                }
                .as_serde_string(),
            )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
