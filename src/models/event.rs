use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Event
{
    pub id: i32,
    pub time: DateTime<Utc>,
    pub name: String,
}

impl Event {
    pub fn new(time: DateTime<Utc>, name: String) -> Self {
        Event{id: 0, time, name}
    }
}
