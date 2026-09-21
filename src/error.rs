use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Kesalahan database: {0}")]
    Database(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DB_ERROR".to_string(),
                    "Terjadi Kesalahan Pada Database",
                )
            }
        };

        let resp = ErrorDetails {
            code: code.to_string(),
            message: message.to_string(),
        };

        (status, Json(resp)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            _ => AppError::Database(err.to_string()),
        }
    }
}
