use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSeriesRequest {
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
    pub color: String,
    pub num_times_notified: i32,
    pub should_notify: bool,
}
