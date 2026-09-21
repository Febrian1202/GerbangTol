use std::ops::Deref;

use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T> Deref for ValidatedJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::BadRequest(format!("Format JSON Request tidak valid: {}", e)))?;

        value.validate().map_err(|e| {
            let details = serde_json::to_value(&e).ok();
            AppError::ValidationError {
                message: String::from("Validasi Input Request Gagal"),
                details,
            }
        })?;

        Ok(ValidatedJson(value))
    }
}
