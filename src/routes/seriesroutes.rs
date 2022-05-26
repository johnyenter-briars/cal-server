use crate::{
    db::calconnector::CalConnector,
    models::server::{
        requests::createseriesrequest::CreateSeriesRequest,
        responses::{createseriesresponse::CreateSeriesResponse, seriesresponse::SeriesResponse},
    },
};
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

#[post("/api/series")]
pub async fn create_series(create_series_req: web::Json<CreateSeriesRequest>) -> HttpResponse {
    match CalConnector::create_series(create_series_req.0) {
        Ok(series_id) => CreateSeriesResponse::created(series_id),
        Err(e) => CreateSeriesResponse::error(e.to_string()),
    }
}

#[get("/api/series/{series_id}")]
pub async fn get_series(series_id: web::Path<String>) -> HttpResponse {
    let uuid = match Uuid::parse_str(&series_id) {
        Ok(id) => id,
        Err(_) => return SeriesResponse::bad_request("Unable to parse UUID".to_string()),
    };

    match CalConnector::get_series(uuid) {
        Ok(option) => match option {
            Some(s) => SeriesResponse::ok(s),
            None => SeriesResponse::not_found(),
        },
        Err(e) => SeriesResponse::error(e.to_string()),
    }
}
