
use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateCalendarRequest {
    pub name: String,
    pub description: Option<String>,
    pub cal_user_id: Uuid,
}
