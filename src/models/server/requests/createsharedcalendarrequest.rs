
use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSharedCalendarRequest {
    pub name: String,
    pub description: Option<String>,
    pub owner_cal_user_id: Uuid,
    pub cal_users: Vec<Uuid>,
    pub color: String,
}
