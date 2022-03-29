use serde::Serialize;

#[derive(Serialize)]
pub struct CreateEventResponse {
    status_code: u32,
    message: String,
}

impl CreateEventResponse {
    pub fn created() -> Self {
        CreateEventResponse {
            status_code: 201,
            message: "Event created".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        CreateEventResponse {
            status_code: 500,
            message,
        }
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
