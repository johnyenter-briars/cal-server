use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventResponse {
    status_code: u32,
    message: String,
    event_id: Option<Uuid>,
}

impl CreateEventResponse {
    pub fn created(id: Uuid) -> Self {
        CreateEventResponse {
            status_code: 201,
            message: "Event created".to_string(),
            event_id: Some(id),
        }
    }

    pub fn error(message: String) -> Self {
        CreateEventResponse {
            status_code: 500,
            message,
            event_id: None,
        }
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
