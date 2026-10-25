use axum::{extract::State, http::StatusCode};

use crate::{
    application::dto::toll_transaction_dto::{TollEntryRequest, TollExitRequest},
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

pub async fn process_exit(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<TollExitRequest>,
) -> Result<StatusCode, AppError> {
    state.toll_transaction_service.process_exit(payload).await?;
    Ok(StatusCode::CREATED)
}
