use actix_web::{get, post, web, HttpResponse};

use crate::{
    models::server::{requests::createnotificationrequest::CreateNotificationRequest, responses::{createnotificationresponse::CreateNotificationResponse, notificationsresponse::NotificationsResponse}},
    server::httpserver::AppState,
};

#[get("/api/notification")]
pub async fn get_notifications(state: web::Data<AppState>) -> HttpResponse {
    let connector = state.cal_connector.lock().unwrap();

    let notifications = match connector.get_all_notifications() {
        Ok(c) => c,
        Err(message) => return NotificationsResponse::error(message.to_string()),
    };

    match notifications.len() {
        0 => NotificationsResponse::not_found(),
        _ => NotificationsResponse::ok(notifications),
    }
}

#[post("/api/notification")]
pub async fn create_notification(
    notification_req: web::Json<CreateNotificationRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let connector = state.cal_connector.lock().unwrap();

    match connector.create_notification(notification_req.0, None) {
        Ok(uuid) => CreateNotificationResponse::created(uuid),
        Err(e) => CreateNotificationResponse::error(e.to_string()),
    }
}