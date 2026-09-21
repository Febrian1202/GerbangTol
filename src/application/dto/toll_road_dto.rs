use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateTollRoadRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
}
