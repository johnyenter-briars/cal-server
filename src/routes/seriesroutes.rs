use crate::{
    models::server::{
        requests::createseriesrequest::CreateSeriesRequest,
        responses::{
            createseriesresponse::CreateSeriesResponse,
            deletedentityresponse::DeletedEntityResponse, seriesresponse::SeriesResponse,
        },
    },
    server::httpserver::AppState,
};
use actix_web::{delete, get, post, web, HttpResponse};
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
        .create_series(create_series_req.0)
    {
        Ok(series_id) => CreateSeriesResponse::created(series_id),
        Err(e) => CreateSeriesResponse::error(e.to_string()),
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
