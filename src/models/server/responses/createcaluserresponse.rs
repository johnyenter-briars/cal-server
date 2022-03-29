use serde::Serialize;

#[derive(Serialize)]
pub struct CreateCalUserResponse {
    status_code: u32,
    message: String,
}

impl CreateCalUserResponse  {
    pub fn created() -> Self {
        CreateCalUserResponse {
            status_code: 201,
            message: "Caluser created".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        CreateCalUserResponse {
            status_code: 500,
            message,
        }
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
