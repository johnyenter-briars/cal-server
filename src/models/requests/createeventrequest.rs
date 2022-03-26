use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub time: DateTime<Utc>,
    pub name: String,
}