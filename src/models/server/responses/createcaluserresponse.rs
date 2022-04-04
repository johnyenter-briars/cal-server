use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCalUserResponse {
    status_code: u32,
    message: String,
    caluser_id: Option<u32>,
}

impl CreateCalUserResponse  {
    pub fn created(id: u32) -> Self {
        CreateCalUserResponse {
            status_code: 201,
            message: "Caluser created".to_string(),
            caluser_id: Some(id),
        }
    }

    pub fn error(message: String) -> Self {
        CreateCalUserResponse {
            status_code: 500,
            message,
            caluser_id: None
        }
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}
