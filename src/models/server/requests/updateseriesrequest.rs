use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

use super::createseriesrequest::CreateSeriesRequest;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSeriesRequest {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub repeat_every_week: u32,
    pub repeat_on_mon: bool,
    pub repeat_on_tues: bool,
    pub repeat_on_wed: bool,
    pub repeat_on_thurs: bool,
    pub repeat_on_fri: bool,
    pub repeat_on_sat: bool,
    pub repeat_on_sun: bool,
    pub starts_on: DateTime<Utc>,
    pub ends_on: DateTime<Utc>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub event_start_time: Duration,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub event_end_time: Duration,
    pub cal_user_id: Uuid,
    pub calendar_id: Uuid,
}

impl UpdateSeriesRequest {
    pub fn to_create_series_request(&self) -> CreateSeriesRequest {
        CreateSeriesRequest {
            name: self.name.clone(),
            description: self.description.clone(),
            repeat_every_week: self.repeat_every_week,
            repeat_on_mon: self.repeat_on_mon,
            repeat_on_tues: self.repeat_on_tues,
            repeat_on_wed: self.repeat_on_wed,
            repeat_on_thurs: self.repeat_on_thurs,
            repeat_on_fri: self.repeat_on_fri,
            repeat_on_sat: self.repeat_on_sat,
            repeat_on_sun: self.repeat_on_sun,
            starts_on: self.starts_on,
            ends_on: self.ends_on,
            event_start_time: self.event_start_time,
            event_end_time: self.event_end_time,
            cal_user_id: self.cal_user_id,
            calendar_id: self.calendar_id,
        }
    }
}
