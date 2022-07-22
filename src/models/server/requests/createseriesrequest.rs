use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSeriesRequest {
    pub name: String,
    pub description: String,
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
}
