use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEventRequest {
    #[serde(rename(deserialize = "Time"))]
    pub time: DateTime<Utc>,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "CalUserId"))]
    pub cal_user_id: u32,
}