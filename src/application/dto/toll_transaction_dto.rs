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
