use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::{gate_type::GateType, vehicle_class::VehicleClass};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TollGate {
    pub id: Uuid,
    pub toll_point_id: Uuid,
    pub code: String,
    pub name: String,
    pub gate_type: GateType,
    pub assigned_vehicle_class: Option<VehicleClass>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
