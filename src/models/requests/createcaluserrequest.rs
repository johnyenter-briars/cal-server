use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCalUserRequest {
    #[serde(rename(deserialize = "FirstName"))]
    pub first_name: String,
    #[serde(rename(deserialize = "LastName"))]
    pub last_name: String,
}