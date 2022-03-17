use serde::Serialize;

#[derive(Serialize)]
pub struct NewEventResponse {
    status_code: u32,
    message: String,
}

impl NewEventResponse {
    pub fn created() -> Self {
        NewEventResponse {
            status_code: 201,
            message: "Event created".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        NewEventResponse {
            status_code: 500,
            message,
        }
    }

    pub fn as_serde_string(self) -> String {
        let response_string =
            serde_json::to_string(&self).expect("Unable to parse response object!");
        response_string
    }
}
