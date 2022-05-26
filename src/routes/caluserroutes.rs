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
pub async fn create_caluser(_: web::Json<CreateCalUserRequest>) -> HttpResponse {
    CreateCalUserResponse::error("This endpoint is deprecated".to_string())
}

#[get("/api/caluser/{user_id}")]
pub async fn get_caluser(user_id: web::Path<String>) -> HttpResponse {
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return CalUserResponse::bad_request("Unable to parse UUID".to_string()),
    };

    match CalConnector::get_caluser(uuid) {
        Ok(option) => match option {
            Some(s) => CalUserResponse::ok(s),
            None => CalUserResponse::not_found(),
        },
        Err(e) => CalUserResponse::error(e.to_string()),
    }
}
