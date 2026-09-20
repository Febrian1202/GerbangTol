use crate::interface::handlers::test_handler;
use axum::Router;
use axum::routing::get;

pub fn create_router() -> Router {
    Router::new().route("/test/{id}", get(test_handler::test_handler))
}
