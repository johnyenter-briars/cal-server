use actix_web::{delete, get, post, web, HttpResponse};
use uuid::Uuid;

use crate::{
    models::{
        cal::calendar::Calendar,
        server::{
            requests::createcalendarrequest::CreateCalendarRequest,
            requests::createsharedcalendarrequest::CreateSharedCalendarRequest,
            responses::{
                calendarsresponse::CalendarsResponse,
                createcalendarresponse::CreateCalendarResponse,
                deletedentityresponse::DeletedEntityResponse,
            },
        },
    },
    server::httpserver::AppState,
};

#[post("/api/calendar")]
pub async fn create_calendar(
    calendar_req: web::Json<CreateCalendarRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let connector = state.cal_connector.lock().unwrap();

    match connector.create_calendar(calendar_req.0, None) {
        Ok(uuid) => CreateCalendarResponse::created(uuid),
        Err(e) => CreateCalendarResponse::error(e.to_string()),
    }
}

#[post("/api/sharedcalendar")]
pub async fn create_shared_calendar(
    calendar_req: web::Json<CreateSharedCalendarRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let connector = state.cal_connector.lock().unwrap();

    match connector.create_shared_calendar(calendar_req.0, None) {
        Ok(uuid) => CreateCalendarResponse::created(uuid),
        Err(e) => CreateCalendarResponse::error(e.to_string()),
    }
}

#[delete("/api/calendar/{uuid}")]
pub async fn delete_calendar(uuid: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let id = Uuid::parse_str(&uuid).expect("uuid improperly formatted");

    match state.cal_connector.lock().unwrap().delete_calendar(id) {
        Ok(option) => match option {
            Some(id) => DeletedEntityResponse::ok(id),
            None => DeletedEntityResponse::bad_request("No entity found with that Id".to_string()),
        },
        Err(e) => DeletedEntityResponse::error(e.to_string()),
    }
}

#[get("/api/calendar/user/{uuid}")]
pub async fn get_calendars_for_user(
    uuid: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let id = Uuid::parse_str(&uuid).expect("uuid improperly formatted");
    let connector = state.cal_connector.lock().unwrap();

    let calendars = match connector.get_calendars_for_user(id) {
        Ok(c) => c,
        Err(message) => return CalendarsResponse::error(message.to_string()),
    };

    match calendars.len() {
        0 => CalendarsResponse::not_found(),
        _ => CalendarsResponse::ok(calendars),
    }
}
