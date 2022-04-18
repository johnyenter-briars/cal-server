use crate::{
    db::calconnector::CalConnector,
    models::server::{
        requests::createcaluserrequest::CreateCalUserRequest,
        responses::{
            caluserresponse::CalUserResponse, createcaluserresponse::CreateCalUserResponse,
        },
    },
};
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

#[post("/api/caluser")]
pub async fn create_caluser(create_user_req: web::Json<CreateCalUserRequest>) -> HttpResponse {
    let result = CalConnector::create_caluser(
        &create_user_req.0.first_name,
        &create_user_req.0.last_name,
        None,
    );

    match result {
        Ok(id) => CreateCalUserResponse::created(id),
        Err(e) => CreateCalUserResponse::error(e.to_string()),
    }
}

#[get("/api/caluser/{user_id}")]
pub async fn get_caluser(user_id: web::Path<String>) -> HttpResponse {
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return CalUserResponse::bad_request("Unable to parse UUID".to_string()),
    };

    let result = CalConnector::get_caluser(uuid);

    match result {
        Ok(user) => CalUserResponse::ok(user),
        Err(e) => CalUserResponse::error(e.to_string()),
    }
}
