use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TollPoint {
    pub id: Uuid,
    pub toll_road_id: Uuid,
    pub code: String,
    pub name: String,
    pub km_position: f64,
    pub sequence_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
