use crate::{
    db::calconnector::CalConnector,
    models::server::{
        responses::{createseriesresponse::CreateSeriesResponse, seriesresponse::SeriesResponse
        }, requests::createseriesrequest::CreateSeriesRequest,
    },
};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse};

#[post("/api/series")]
pub async fn create_series(create_series_req: web::Json<CreateSeriesRequest>) -> HttpResponse {
    let result =
        CalConnector::create_series(create_series_req.0);

    match result {
        Ok(series_id) => HttpResponse::Created()
            .content_type(ContentType::json())
            .body(CreateSeriesResponse::created(series_id).as_serde_string()),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(CreateSeriesResponse::error(e.to_string()).as_serde_string()),
    }
}

#[get("/api/series/{series_id}")]
pub async fn get_series(series_id: web::Path<String>) -> HttpResponse {
    let result = CalConnector::get_series(series_id.parse::<u32>().unwrap());

    match result {
        Ok(s) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(SeriesResponse::ok(s).as_serde_string()),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(SeriesResponse::error(e.to_string()).as_serde_string()),
    }
}
