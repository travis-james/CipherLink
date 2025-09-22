use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    app_config::AppConfig,
    crypto::{decrypt, encrypt},
    db::{self, DynamoDBClient},
    transformer::{encrypt_data_to_item, item_to_encryt_data},
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
    id: String,
}

#[derive(Serialize)]
#[serde(tag = "status", content = "data")]
enum EncryptApiResponse {
    Ok(EncryptResponse),
    Err(ErrorResponse),
}

#[derive(Debug, Deserialize)]
struct DecryptParams {
    id: String,
    key: String,
}

pub async fn init(config: AppConfig) {
    let db_client = db::init(&config.db_url, &config.region).await;

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/encrypt", post(encrypt_handler))
        .route("/decrypt/{id}/{key}", get(decrypt_handler))
        .layer(Extension(db_client));

    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
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
        Ok(encrypted_data) => encrypted_data,
        Err(_) => {
            return Json(EncryptApiResponse::Err(ErrorResponse {
                error: "Encryption failed".to_string(),
            }));
        }
    };

    let id = Uuid::new_v4().to_string();
    let item = encrypt_data_to_item(&id, &encrypted_data);

    if let Err(e) = db_client.insert_item("encryptData", item).await {
        return Json(EncryptApiResponse::Err(ErrorResponse {
            error: format!("DB insert failed: {}", e),
        }));
    }

    Json(EncryptApiResponse::Ok(EncryptResponse { id }))
}

async fn decrypt_handler(
    Extension(db_client): Extension<DynamoDBClient>,
    Path(params): Path<DecryptParams>,
) -> Response {
    // Get the data.
    let data = match db_client.get("encryptData", "id", &params.id).await {
        Ok(data) => data,
        Err(e) => {
            return Json(ErrorResponse {
                error: format!("DB get failed: {}", e),
            })
            .into_response();
        }
    };

    // Decode the data from dynamodb.
    let transformed_data = match item_to_encryt_data(&data) {
        Ok(transformed_data) => transformed_data,
        Err(e) => {
            return Json(ErrorResponse {
                error: format!("Failed to transform: {}", e),
            })
            .into_response();
        }
    };

    // Now decrypt retrieved data.
    let decrypted_data = match decrypt(&transformed_data, &params.key) {
        Ok(decrypted_data) => decrypted_data,
        Err(e) => {
            return Json(ErrorResponse {
                error: format!("Failed to decrypt: {}", e),
            })
            .into_response();
        }
    };

    // Delete data from dynamo db.
    if let Err(e) = db_client.delete("encryptData", "id", &params.id).await {
        return Json(ErrorResponse {
            error: format!("Failed to delete: {}", e),
        })
        .into_response();
    }

    // Finally redirect.
    let str_data = String::from_utf8_lossy(&decrypted_data).to_string();
    match Url::parse(&str_data) {
        Ok(valid_url) => Redirect::temporary(valid_url.as_str()).into_response(),
        Err(_) => Json(ErrorResponse {
            error: "Decrypted data is not a valid URL".to_string(),
        })
        .into_response(),
    }
}
