use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Data tidak ditemukan: {0}")]
    NotFound(String),

    #[error("Konflik data: {0}")]
    Conflict(String),

    #[error("Permintaan data tidak valid: {0}")]
    BadRequest(String),

    #[error("Validasi input gagal: {message}")]
    ValidationError {
        message: String,
        details: Option<serde_json::Value>,
    },

    #[error("Kesalahan database: {0}")]
    Database(String),

    #[error("Kesalahan internal sistem: {0}")]
    Internal(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, details) = match self {
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, String::from("NOT_FOUND"), msg, None)
            }
            AppError::Conflict(msg) => (StatusCode::CONFLICT, String::from("CONFLICT"), msg, None),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                String::from("BAD_REQUEST"),
                msg,
                None,
            ),
            AppError::ValidationError { message, details } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                String::from("VALIDATION_ERROR"),
                message,
                details,
            ),
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("DB_ERROR"),
                    String::from("Terjadi Kesalahan Pada Database"),
                    None,
                )
            }
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("INTERNAL_ERROR"),
                msg,
                None,
            ),
        };

        let resp = ErrorDetails {
            code,
            message,
            details,
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
