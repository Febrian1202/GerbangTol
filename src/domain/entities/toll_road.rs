use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::toll_point::TollPoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TollRoad {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub points: Vec<TollPoint>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
