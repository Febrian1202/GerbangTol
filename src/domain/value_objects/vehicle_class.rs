use serde::{Deserialize, Serialize};

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
