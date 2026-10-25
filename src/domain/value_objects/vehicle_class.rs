use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VehicleClass {
    #[serde(rename = "GOLONGAN_1")]
    Golongan1,
    #[serde(rename = "GOLONGAN_2")]
    Golongan2,
}

impl VehicleClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            VehicleClass::Golongan1 => "GOLONGAN_1",
            VehicleClass::Golongan2 => "GOLONGAN_2",
        }
    }
}

impl FromStr for VehicleClass {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "GOLONGAN_1" => Ok(VehicleClass::Golongan1),
            "GOLONGAN_2" => Ok(VehicleClass::Golongan2),
            _ => Err(AppError::BadRequest("Invalid golongan kendaraan".into())),
        }
    }
}
