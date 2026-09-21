use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::toll_road::TollRoad;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTollRoadRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TollRoadResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TollRoad> for TollRoadResponse {
    fn from(road: TollRoad) -> Self {
        Self {
            id: road.id,
            code: road.code,
            name: road.name,
            description: road.description,
            created_at: road.created_at,
            updated_at: road.updated_at,
        }
    }
}
