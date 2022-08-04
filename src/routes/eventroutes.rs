use crate::{
    db::calconnector::CalConnector,
    models::{
        server::{
            requests::{
                createeventrequest::CreateEventRequest, updateeventrequest::UpdateEventRequest,
            },
            responses::{
                createeventresponse::CreateEventResponse, eventsresponse::EventsResponse,
                updateeventresponse::UpdateEventResponse,
            },
        },
        traits::validate::Validatable,
    },
    server::httpserver::AppState,
};
use actix_web::{get, post, put, web, HttpResponse};

#[post("/api/event")]
pub async fn create_event(
    create_event_req: web::Json<CreateEventRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    create_event_base(create_event_req.0, &state.cal_connector.lock().unwrap())
}

#[put("/api/event")]
pub async fn update_event(
    update_event_req: web::Json<UpdateEventRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let cal_connector = state.cal_connector.lock().unwrap();
    let events = match cal_connector.get_events() {
        Ok(e) => e,
        Err(e) => {
            return UpdateEventResponse::error(e.to_string());
        }
    };

    if !events.iter().any(|e| e.id == update_event_req.id) {
        let create_event_req = CreateEventRequest {
            start_time: update_event_req.start_time,
            end_time: update_event_req.end_time,
            name: update_event_req.name.clone(),
            description: update_event_req.description.clone(),
            cal_user_id: update_event_req.cal_user_id,
            series_id: update_event_req.series_id,
        };

        return create_event_base(create_event_req, &cal_connector);
    }

    let (pass, message) = validate_request(&update_event_req.0);

    if !pass {
        return UpdateEventResponse::bad_request(message);
    }

    match cal_connector.update_event(update_event_req.0) {
        Ok(id) => UpdateEventResponse::updated(id),
        Err(e) => UpdateEventResponse::error(e.to_string()),
    }
}

#[get("/api/event")]
pub async fn get_events(state: web::Data<AppState>) -> HttpResponse {
    match state.cal_connector.lock().unwrap().get_events() {
        Ok(events) => EventsResponse::ok(events),
        Err(e) => EventsResponse::error(e.to_string()),
    }
}

fn create_event_base(
    create_event_req: CreateEventRequest,
    cal_connector: &CalConnector,
) -> HttpResponse {
    let (pass, message) = validate_request(&create_event_req);

    if !pass {
        return CreateEventResponse::bad_request(message);
    }

    match cal_connector.create_event(create_event_req, None) {
        Ok(id) => CreateEventResponse::created(id),
        Err(e) => CreateEventResponse::error(e.to_string()),
    }
}

fn validate_request(event_req: &dyn Validatable) -> (bool, String) {
    match (event_req.time_is_populated(), event_req.end_after_start()) {
        ((true, _), (true, _)) => (true, "".to_string()),
        ((false, m1), (true, _)) => (false, m1),
        ((true, _), (false, m2)) => (false, m2),
        ((false, m1), (false, _)) => (false, m1),
    }
}
