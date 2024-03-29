use crate::{
    models::server::{
        requests::createcaluserrequest::CreateCalUserRequest,
        responses::{
            caluserresponse::CalUserResponse, createcaluserresponse::CreateCalUserResponse,
        },
    },
    server::httpserver::AppState,
};
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

#[post("/api/caluser")]
pub async fn create_caluser(_: web::Json<CreateCalUserRequest>) -> HttpResponse {
    CreateCalUserResponse::error("This endpoint is deprecated".to_string())
}

#[get("/api/caluser/{user_id}")]
pub async fn get_caluser(user_id: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return CalUserResponse::bad_request("Unable to parse UUID".to_string()),
    };

    match state.cal_connector.lock().unwrap().get_caluser(uuid) {
        Ok(option) => match option {
            Some(s) => CalUserResponse::ok(s),
            None => CalUserResponse::not_found(),
        },
        Err(e) => CalUserResponse::error(e.to_string()),
    }
}
