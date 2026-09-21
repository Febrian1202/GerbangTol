use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};

use crate::interface::state::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResult {
    status: String,
}

pub async fn test_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<TestResult> {
    Json(TestResult { status: id })
}
