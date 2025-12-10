use axum::{Router, routing::get};
use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/foo", get(handlers::get).post(handlers::post))
        .route("/foo/bar", get(handlers::bar))
}

