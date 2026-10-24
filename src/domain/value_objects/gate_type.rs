use std::{io::ErrorKind::Other, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GateType {
    Entry,
    Exit,
}

impl GateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GateType::Entry => "Entry",
            GateType::Exit => "Exit",
        }
    }
}

impl FromStr for GateType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "ENTRY" => Ok(GateType::Entry),
            "EXIT" => Ok(GateType::Exit),
            other => Err(format!("Unknown gate type: {}", other)),
        }
    }
}
