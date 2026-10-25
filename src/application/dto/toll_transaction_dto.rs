use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct TollEntryRequest {
    pub gate_id: Uuid,

    #[validate(length(
        min = 3,
        max = 15,
        message = "Format nomor plate kendaraan tidak valid"
    ))]
    pub plate_number: String,
}

#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct TollExitRequest {
    pub gate_id: Uuid,

    #[validate(length(
        min = 3,
        max = 15,
        message = "Format nomor plate kendaraan tidak valid"
    ))]
    pub plate_number: String,
}

#[derive(Debug, Clone)]
pub struct CalculationResult {
    pub distance_km: f64,
    pub base_fare: f64,
    pub rate_per_km: f64,
    pub total_fee: f64,
    pub receipt_hash: String,
    pub elapsed_nanos: u128,
}
