use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::traits::validate::Validatable;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequest {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub cal_user_id: Uuid,
    pub series_id: Option<Uuid>,
}

impl Validatable for CreateEventRequest {
    fn time_is_populated(&self) -> (bool, String) {
        if !(self.start_time.is_some() && self.end_time.is_some()) {
            return (
                false,
                "You must construct an event with both start and end both Some".to_string(),
            );
        }

        (true, "".to_string())
    }

    fn end_after_start(&self) -> (bool, String) {
        if self.start_time > self.end_time {
            return (false, "End time may not be after start time".to_string());
        }

        (true, "".to_string())
    }
}
