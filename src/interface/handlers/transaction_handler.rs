use axum::{extract::State, http::StatusCode};

use crate::{
    application::dto::toll_transaction_dto::TollEntryRequest,
    error::AppError,
    interface::{extractors::ValidatedJson, state::AppState},
};

pub async fn process_entry(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<TollEntryRequest>,
) -> Result<StatusCode, AppError> {
    state
        .toll_transaction_service
        .process_entry(payload)
        .await?;

    Ok(StatusCode::CREATED)
}
