use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Kesalahan database: {0}")]
    Database(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            _ => AppError::Database(err.to_string()),
        }
    }
}
