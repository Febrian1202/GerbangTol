use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::{
    transaction_status::TransactionStatus, vehicle_class::VehicleClass,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TollTransaction {
    pub id: Uuid,
    pub plate_number: String,
    pub vehicle_class: VehicleClass,
    pub entry_toll_point_id: Uuid,
    pub entry_gate_id: Uuid,
    pub entry_time: DateTime<Utc>,
    pub entry_km: f64,
    pub exit_toll_point_id: Option<Uuid>,
    pub exit_gate_id: Option<Uuid>,
    pub exit_time: Option<DateTime<Utc>>,
    pub exit_km: Option<f64>,
    pub distance_km: Option<f64>,
    pub base_fare: Option<f64>,
    pub rate_per_km: Option<f64>,
    pub total_fee: Option<f64>,
    pub status: TransactionStatus,
    pub receipt_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
