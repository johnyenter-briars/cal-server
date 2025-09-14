use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotificationRequest {
    pub calendar_id: Uuid,
    pub event_id: Uuid,
    pub cal_user_id: Uuid,
}
