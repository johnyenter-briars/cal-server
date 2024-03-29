use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

use crate::{
    models::server::responses::{
        listsavesresponse::ListSavesResponse, loaddatabaseresponse::LoadDatabaseResponse,
        savedatabaseresponse::SaveDatabaseResult,
    },
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

#[get("/api/admin/database/list")]
pub async fn list_database_saves(state: web::Data<AppState>) -> HttpResponse {
    let connector = state.cal_connector.lock().unwrap();

    match connector.list_database_saves() {
        Ok(saves) => ListSavesResponse::ok(saves),
        Err(e) => ListSavesResponse::error(e.to_string()),
    }
}

#[post("/api/admin/database/load/{uuid}")]
pub async fn load_database_version(
    uuid: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let connector = state.cal_connector.lock().unwrap();

    let id = Uuid::parse_str(&uuid).expect("uuid improperly formatted");

    match connector.load_database_save(id) {
        Ok(_) => LoadDatabaseResponse::ok(),
        Err(e) => LoadDatabaseResponse::error(e.to_string()),
    }
}
