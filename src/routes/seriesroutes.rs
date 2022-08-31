use crate::{
    models::server::{
        requests::{
            createseriesrequest::CreateSeriesRequest, updateseriesrequest::UpdateSeriesRequest,
        },
        responses::{
            allseriesresponse::AllSeriesResponse, createseriesresponse::CreateSeriesResponse,
            deletedentityresponse::DeletedEntityResponse, seriesresponse::SeriesResponse,
            updateseriesresponse::UpdateSeriesResponse,
        },
    },
    server::httpserver::AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

#[post("/api/series")]
pub async fn create_series(
    create_series_req: web::Json<CreateSeriesRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match state
        .cal_connector
        .lock()
        .unwrap()
        .create_series(create_series_req.0, None)
    {
        Ok(series_id) => CreateSeriesResponse::created(series_id),
        Err(e) => CreateSeriesResponse::error(e.to_string()),
    }
}

#[get("/api/series")]
pub async fn get_all_series(state: web::Data<AppState>) -> HttpResponse {
    match state.cal_connector.lock().unwrap().get_all_series() {
        Ok(s) => AllSeriesResponse::ok(s),
        Err(e) => AllSeriesResponse::error(e.to_string()),
    }
}

#[put("/api/series")]
pub async fn update_series(
    update_series_req: web::Json<UpdateSeriesRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let cal_connector = state.cal_connector.lock().unwrap();
    let serieses = match cal_connector.get_all_series() {
        Ok(e) => e,
        Err(e) => {
            return UpdateSeriesResponse::error(e.to_string());
        }
    };

    if !serieses.iter().any(|s| s.id == update_series_req.id) {
        let create_series_req = CreateSeriesRequest {
            name: update_series_req.name.clone(),
            description: update_series_req.description.clone(),
            repeat_every_week: update_series_req.repeat_every_week,
            repeat_on_mon: update_series_req.repeat_on_mon,
            repeat_on_tues: update_series_req.repeat_on_tues,
            repeat_on_wed: update_series_req.repeat_on_wed,
            repeat_on_thurs: update_series_req.repeat_on_thurs,
            repeat_on_fri: update_series_req.repeat_on_fri,
            repeat_on_sat: update_series_req.repeat_on_sat,
            repeat_on_sun: update_series_req.repeat_on_sun,
            starts_on: update_series_req.starts_on,
            ends_on: update_series_req.ends_on,
            event_start_time: update_series_req.event_start_time,
            event_end_time: update_series_req.event_end_time,
            cal_user_id: update_series_req.cal_user_id,
            calendar_id: update_series_req.calendar_id,
        };

        return match cal_connector.create_series(create_series_req, None) {
            Ok(series_id) => CreateSeriesResponse::created(series_id),
            Err(e) => CreateSeriesResponse::error(e.to_string()),
        };
    }

    match cal_connector.update_series(update_series_req.0) {
        Ok(id) => UpdateSeriesResponse::updated(id),
        Err(e) => UpdateSeriesResponse::error(e.to_string()),
    }
}

#[get("/api/series/{series_id}")]
pub async fn get_series(series_id: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let uuid = match Uuid::parse_str(&series_id) {
        Ok(id) => id,
        Err(_) => return SeriesResponse::bad_request("Unable to parse UUID".to_string()),
    };

    match state.cal_connector.lock().unwrap().get_series(uuid) {
        Ok(option) => match option {
            Some(s) => SeriesResponse::ok(s),
            None => SeriesResponse::not_found(),
        },
        Err(e) => SeriesResponse::error(e.to_string()),
    }
}

#[delete("/api/series/{uuid}")]
pub async fn delete_series(uuid: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let id = Uuid::parse_str(&uuid).expect("uuid improperly formatted");

    match state.cal_connector.lock().unwrap().delete_series(id) {
        Ok(option) => match option {
            Some(id) => DeletedEntityResponse::ok(id),
            None => DeletedEntityResponse::bad_request("No entity found with that Id".to_string()),
        },
        Err(e) => DeletedEntityResponse::error(e.to_string()),
    }
}
