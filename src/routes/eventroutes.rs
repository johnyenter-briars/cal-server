use crate::{
    db::calconnector::CalConnector,
    models::{
        cal::event::Event,
        server::{
            requests::{
                createeventrequest::CreateEventRequest, updateeventrequest::UpdateEventRequest,
            },
            responses::{
                createeventresponse::CreateEventResponse,
                deletedentityresponse::DeletedEntityResponse, eventsresponse::EventsResponse,
                updateeventresponse::UpdateEventResponse,
            },
        },
        traits::validate::Validatable,
    },
    server::httpserver::AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use itertools::Itertools;
use uuid::Uuid;

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
            calendar_id: update_event_req.calendar_id,
            color: update_event_req.color.clone(),
            num_times_notified: update_event_req.num_times_notified,
            should_notify: update_event_req.should_notify,
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

#[get("/api/event/{name}")]
pub async fn get_event_via_name(
    name: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match state
        .cal_connector
        .lock()
        .unwrap()
        .get_events_via_name(name.to_string())
    {
        Ok(events) => EventsResponse::ok(events),
        Err(e) => EventsResponse::error(e.to_string()),
    }
}

#[get("/api/event/page/{page}")]
pub async fn get_event_page(page: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    match state.cal_connector.lock().unwrap().get_events() {
        Ok(events) => {
            let chunked_items: Vec<Vec<Event>> = events
                .into_iter()
                .chunks(20)
                .into_iter()
                .map(|chunk| chunk.collect())
                .collect();

            let p: usize = page.parse::<usize>().unwrap();

            if p >= chunked_items.len() {
                return EventsResponse::ok(vec![]);
            }

            EventsResponse::ok(chunked_items[p].clone())
        }
        Err(e) => EventsResponse::error(e.to_string()),
    }
}

#[get("/api/event/{year}/{month}")]
pub async fn get_events_of_month(
    params: web::Path<(i32, u32)>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match state
        .cal_connector
        .lock()
        .unwrap()
        .get_events_month_year(params.0, params.1)
    {
        Ok(events) => EventsResponse::ok(events),
        Err(e) => EventsResponse::error(e.to_string()),
    }
}

#[delete("/api/event/{uuid}")]
pub async fn delete_event(uuid: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let id = Uuid::parse_str(&uuid).expect("uuid improperly formatted");

    match state
        .cal_connector
        .lock()
        .unwrap()
        .delete_entity(id, "event")
    {
        Ok(option) => match option {
            Some(id) => DeletedEntityResponse::ok(id),
            None => DeletedEntityResponse::bad_request("No entity found with that Id".to_string()),
        },
        Err(e) => DeletedEntityResponse::error(e.to_string()),
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
