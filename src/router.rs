use axum::{Router, routing::get};
use crate::handlers::foo;

pub fn create_router() -> Router {
    Router::new()
        .route("/foo", get(foo::get).post(foo::post))
        .route("/foo/bar", get(foo::bar))
}

