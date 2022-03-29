use crate::{
    db::calconnector::CalConnector,
    models::server::{
        requests::createcaluserrequest::CreateCalUserRequest,
        responses::{
            caluserresponse::CalUserResponse, createcaluserresponse::CreateCalUserResponse,
        },
    },
};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse};

#[post("/api/caluser")]
pub async fn create_caluser(create_user_req: web::Json<CreateCalUserRequest>) -> HttpResponse {
    let result =
        CalConnector::create_caluser(&create_user_req.0.first_name, &create_user_req.0.last_name);

    match result {
        Ok(_) => HttpResponse::Created()
            .content_type(ContentType::json())
            .body(CreateCalUserResponse::created().as_serde_string()),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(CreateCalUserResponse::error(e.to_string()).as_serde_string()),
    }
}

#[get("/api/caluser/{user_id}")]
pub async fn get_caluser(user_id: web::Path<String>) -> HttpResponse {
    let result = CalConnector::get_caluser(user_id.parse::<u32>().unwrap());

    match result {
        Ok(u) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(CalUserResponse::ok(u).as_serde_string()),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(CalUserResponse::error(e.to_string()).as_serde_string()),
    }
}
