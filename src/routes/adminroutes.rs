use actix_web::{http::header::ContentType, post, web, HttpResponse};

use crate::{
    models::server::responses::savedatabaseresponse::SaveDatabaseResult,
    server::httpserver::AppState,
};

#[post("/api/admin/database/save")]
pub async fn save_database(state: web::Data<AppState>) -> HttpResponse {
    let connector = state.cal_connector.lock().unwrap();

    match connector.save_database() {
        Ok(uuid) => SaveDatabaseResult::ok(uuid),
        Err(e) => SaveDatabaseResult::error(e.to_string()),
    }
}

#[post("/api/admin/database/list")]
pub async fn list_database_saves() -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type(ContentType::plaintext())
        .body("test")
}

#[post("/api/admin/database/load/{uuid}")]
pub async fn load_database_version(uuid: web::Path<String>) -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type(ContentType::plaintext())
        .body("test")
}
