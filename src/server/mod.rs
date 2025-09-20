use axum::{
    http::StatusCode, response::IntoResponse, routing::get, Json, Router
};
use serde::Serialize;

pub async fn init() {
    let app = Router::new().route("/health", get(health));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthStatus { status: "healthy" }))
}