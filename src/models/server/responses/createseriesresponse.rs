use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSeriesResponse {
    status_code: u32,
    message: String,
    series_id: Option<u32>,
}

impl CreateSeriesResponse {
    pub fn created(series_id: u32) -> Self {
        CreateSeriesResponse {
            status_code: 201,
            message: "Event created".to_string(),
            series_id: Some(series_id),
        }
    }

    pub fn error(message: String) -> Self {
        CreateSeriesResponse {
            status_code: 500,
            message,
            series_id: None
        }
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}