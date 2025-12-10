mod router;
mod handlers;

use router::create_router;

use axum::{
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().merge(create_router());

    println!("Server running on port http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}