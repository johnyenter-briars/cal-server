use crate::models::cal::caluser::CalUser;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalUserResponse { 
    user: Option<CalUser>,
    status_code: u32,
    message: String,
}

impl CalUserResponse  {
    pub fn ok(user: CalUser) -> Self {
        CalUserResponse{status_code: 200, message: "CalUser found".to_string(), user: Some(user)}
    }
    
    pub fn bad_request(message: String) -> Self {
        CalUserResponse{status_code: 400, message, user: None }
    }

    pub fn error(message: String) -> Self {
        CalUserResponse{status_code: 500, message, user: None}
    }
    
    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}