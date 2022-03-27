use crate::models::caluser::CalUser;
use serde::Serialize;

#[derive(Serialize)]
pub struct CalUserResponse { 
    user: Option<CalUser>,
    status_code: u32,
    message: String,
}

impl CalUserResponse  {
    pub fn ok(user: CalUser) -> Self {
        CalUserResponse{status_code: 200, message: "CalUser found".to_string(), user: Some(user)}
    }

    pub fn error(message: String) -> Self {
        CalUserResponse{status_code: 500, message, user: None}
    }
    
    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}