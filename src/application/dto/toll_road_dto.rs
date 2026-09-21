use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::entities::{toll_point::TollPoint, toll_road::TollRoad};

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateTollRoadRequest {
    #[validate(length(min = 1, max = 50, message = "Kode toll harus antara 1-50 karakter"))]
    pub code: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Nama gerbang harus antara 1-255 karakter"
    ))]
    pub name: String,

    pub description: Option<String>,

    #[validate(nested)]
    #[serde(default)]
    pub points: Vec<CreateTollPointDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateTollPointDto {
    #[validate(length(min = 1, max = 50, message = "Kode titik harus antara 1-50 karakter"))]
    pub code: String,

    #[validate(length(min = 1, max = 255, message = "Nama titik harus antara 1-255 karakter"))]
    pub name: String,

    #[validate(range(min = 0.0, message = "Posisi kilometer harus lebih atau sama dengan 0"))]
    pub km_position: f64,
    pub sequence_order: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TollRoadResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub points: Vec<TollPointResponse>,
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
            points: road.points.into_iter().map(Into::into).collect(),
            created_at: road.created_at,
            updated_at: road.updated_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TollPointResponse {
    pub id: Uuid,
    pub toll_road_id: Uuid,
    pub code: String,
    pub name: String,
    pub km_position: f64,
    pub sequence_order: i32,
}

impl From<TollPoint> for TollPointResponse {
    fn from(point: TollPoint) -> Self {
        Self {
            id: point.id,
            toll_road_id: point.toll_road_id,
            code: point.code,
            name: point.name,
            km_position: point.km_position,
            sequence_order: point.sequence_order,
        }
    }
}
