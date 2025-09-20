use axum::{
    Extension, Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    crypto::encrypt,
    db::{self, DynamoDBClient},
    transformer::encrypt_data_to_item,
};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
}

#[derive(Deserialize)]
struct EncryptRequest {
    plain_text: String,
    key: String,
}

#[derive(Serialize)]
struct EncryptResponse {
    your_key: String,
}

#[derive(Serialize)]
#[serde(tag = "status", content = "data")]
enum EncryptApiResponse {
    Ok(EncryptResponse),
    Err(ErrorResponse),
}


pub async fn init() {
    let url = "http://localhost:8000";
    let region = "us-west-2";
    let db_client = db::init(url, region).await;

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/encrypt", post(encrypt_handler))
        .layer(Extension(db_client));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthStatus { status: "healthy" }))
}

async fn encrypt_handler(
    Extension(db_client): Extension<DynamoDBClient>,
    Json(payload): Json<EncryptRequest>,
) -> impl IntoResponse {
    let encrypted_data = match encrypt(&payload.plain_text, &payload.key) {
        Ok(data) => data,
        Err(_) => return Json(EncryptApiResponse::Err(ErrorResponse {
            error: "Encryption failed".to_string(),
        })),
    };

    let id = Uuid::new_v4().to_string();
    let item = encrypt_data_to_item(&id, &encrypted_data);

    if let Err(e) = db_client.insert_item("encryptData", item).await {
        return Json(EncryptApiResponse::Err(ErrorResponse {
            error: format!("DB insert failed: {}", e),
        }));
    }

    Json(EncryptApiResponse::Ok(EncryptResponse { your_key: id }))
}

