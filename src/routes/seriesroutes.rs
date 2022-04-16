use crate::{
    db::calconnector::CalConnector,
    models::server::{
        responses::{createseriesresponse::CreateSeriesResponse, seriesresponse::SeriesResponse
        }, requests::createseriesrequest::CreateSeriesRequest,
    },
};
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

#[post("/api/series")]
pub async fn create_series(create_series_req: web::Json<CreateSeriesRequest>) -> HttpResponse {
    let result =
        CalConnector::create_series(create_series_req.0);

    match result {
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

    let result = CalConnector::get_series(uuid);

    match result {
        Ok(s) => SeriesResponse::ok(s),
        Err(e) => SeriesResponse::error(e.to_string()),
    }
}
