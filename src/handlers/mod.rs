use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    "GET /foo"
}

pub async fn post() -> impl IntoResponse {
    "POST /foo"
}

pub async fn bar() -> impl IntoResponse {
    "GET /foo/bar"
}
