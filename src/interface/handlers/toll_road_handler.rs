use axum::{Json, extract::State};

use crate::{
    application::dto::toll_road_dto::{CreateTollRoadRequest, TollRoadResponse},
    error::AppError,
    interface::{extractors::ValidatedJson, state::AppState},
};

pub async fn create(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateTollRoadRequest>,
) -> Result<Json<TollRoadResponse>, AppError> {
    let response = state.toll_road_service.create(payload).await?;

    Ok(Json(response))
}
