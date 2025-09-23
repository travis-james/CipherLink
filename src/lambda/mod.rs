use lambda_http::{Response, Body};

use crate::{handlers::health_handler, types::HealthStatus};

pub async fn lambda_health_handler() -> Response<Body> {
    let status: HealthStatus = health_handler().await;
    let body = serde_json::to_string(&status).unwrap_or_else(|_| "{}".to_string());

    Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body.into())
        .unwrap()
}