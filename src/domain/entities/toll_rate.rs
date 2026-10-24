use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::vehicle_class::VehicleClass;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TollRate {
    pub id: Uuid,
    pub toll_road_id: Uuid,
    pub vehicle_class: VehicleClass,
    pub base_fare: f64,
    pub rate_per_km: f64,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
