use actix_web::{get, web, HttpResponse};

use crate::{
    models::server::responses::notificationsresponse::NotificationsResponse,
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
