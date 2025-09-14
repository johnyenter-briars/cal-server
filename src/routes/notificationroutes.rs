use actix_web::{delete, get, post, web, HttpResponse};
use uuid::Uuid;

use crate::{
    models::server::{
        requests::createnotificationrequest::CreateNotificationRequest,
        responses::{
            createnotificationresponse::CreateNotificationResponse,
            deletedentityresponse::DeletedEntityResponse,
            notificationsresponse::NotificationsResponse,
        },
    },
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

#[get("/api/notification/event/{uuid}")]
pub async fn get_notifications_for_event(
    uuid: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let id = Uuid::parse_str(&uuid).expect("uuid improperly formatted");

    let connector = state.cal_connector.lock().unwrap();

    let notifications = match connector.get_notifications_for_event(id) {
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

#[delete("/api/notification/{uuid}")]
pub async fn delete_notification(
    uuid: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let id = Uuid::parse_str(&uuid).expect("uuid improperly formatted");

    match state
        .cal_connector
        .lock()
        .unwrap()
        .delete_entity(id, "notification")
    {
        Ok(option) => match option {
            Some(id) => DeletedEntityResponse::ok(id),
            None => DeletedEntityResponse::bad_request("No entity found with that Id".to_string()),
        },
        Err(e) => DeletedEntityResponse::error(e.to_string()),
    }
}
