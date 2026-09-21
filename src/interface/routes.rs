use crate::interface::handlers::test_handler;
use crate::interface::state::AppState;
use axum::Router;
use axum::routing::get;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/test/{id}", get(test_handler::test_handler))
        .with_state(state)
}
