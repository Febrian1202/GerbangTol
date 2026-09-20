use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResult {
    status: String,
}

pub async fn test_handler(Path(id): Path<String>) -> Json<TestResult> {
    Json(TestResult { status: id })
}
