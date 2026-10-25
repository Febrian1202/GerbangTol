use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Active,
    Completed,
}

impl TransactionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionStatus::Active => "ACTIVE",
            TransactionStatus::Completed => "COMPLETED",
        }
    }
}

impl FromStr for TransactionStatus {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "ACTIVE" => Ok(TransactionStatus::Active),
            "COMPLETED" => Ok(TransactionStatus::Completed),
            _ => Err(AppError::BadRequest("Unknown Transaction Status".into())),
        }
    }
}
