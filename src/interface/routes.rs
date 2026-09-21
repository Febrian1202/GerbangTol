use crate::interface::handlers::{test_handler, toll_road_handler};
use crate::interface::state::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/test/{id}", get(test_handler::test_handler))
        .route("/toll-roads", post(toll_road_handler::create))
        .with_state(state)
}
