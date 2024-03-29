use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateCalUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub api_key: String,
}
