use crate::models::cal::{event::Event, series::Series};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesResponse { 
    series: Option<Series>,
    status_code: u32,
    message: String,
}

impl SeriesResponse {
    pub fn ok(series: Series) -> Self {
        SeriesResponse{status_code: 200, message: "Series found".to_string(), series: Some(series)}
    }

    pub fn error(message: String) -> Self {
        SeriesResponse{status_code: 500, message, series: None}
    }
    
    pub fn as_serde_string(self) -> String {
        let response_string =
            serde_json::to_string(&self).expect("Unable to parse response object!");
        response_string
    }
}
