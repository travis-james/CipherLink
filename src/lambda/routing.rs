use http::StatusCode;
use lambda_http::{Body, Request, Response};
use lambda_runtime::Error;

use crate::{
    db::DynamoDBClient,
    handlers::{decrypt_handler, encrypt_handler, health_handler},
    lambda::helpers::{error_payload, extract_body_string, json_response, redirect_response},
    types::{EncryptRequest, HealthStatus},
};

/// Minimal request dispatcher for AWS Lambda.
///
/// Matches incoming HTTP method and path to the
/// appropriate handler.
/// Not a full-featured router—just manual pattern matching..
pub async fn router(event: Request, db_client: &DynamoDBClient) -> Result<Response<Body>, Error> {
    let path = event.uri().path();
    let method = event.method().as_str();

    let resp = match (method, path) {
        ("GET", "/health") => lambda_health_handler().await,
        ("POST", "/encrypt") => lambda_encrypt_handler(event, db_client).await,
        _ if path.starts_with("/decrypt/") => lambda_decrypt_handler(path, db_client).await,
        _ => json_response(&error_payload("Not Found"), StatusCode::NOT_FOUND),
    };

    Ok(resp)
}

/// Lambda wrapper for health_handler.
pub async fn lambda_health_handler() -> Response<Body> {
    let status: HealthStatus = health_handler().await;
    let body = serde_json::to_string(&status).unwrap_or_else(|_| "{}".to_string());

    json_response(&body, StatusCode::OK)
}

/// Lambda wrapper for encrypt_handler.
pub async fn lambda_encrypt_handler(event: Request, db_client: &DynamoDBClient) -> Response<Body> {
    let body_string = match extract_body_string(event.body()) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    let payload: EncryptRequest = match serde_json::from_str(&body_string) {
        Ok(p) => p,
        Err(_) => return json_response(&error_payload("Invalid JSON"), StatusCode::BAD_REQUEST),
    };

    match encrypt_handler(db_client, payload).await {
        Ok(resp) => json_response(&resp, StatusCode::OK),
        Err(err) => json_response(&err, StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Lambda wrapper for decrypt_handler.
pub async fn lambda_decrypt_handler(path: &str, db_client: &DynamoDBClient) -> Response<Body> {
    let parts: Vec<&str> = path.trim_start_matches("/decrypt/").split('/').collect();
    if parts.len() != 2 {
        return json_response(
            &error_payload("Invalid decrypt path"),
            StatusCode::BAD_REQUEST,
        );
    }
    let id = parts[0].to_string();
    let key = parts[1].to_string();
    match decrypt_handler(db_client, id , key).await {
        Ok(url) => match url::Url::parse(&url) {
            Ok(valid_url) => redirect_response(valid_url.as_str()),
            Err(_) => json_response(&error_payload("Invalid URL"), StatusCode::BAD_REQUEST),
        },
        Err(err) => json_response(&err, StatusCode::INTERNAL_SERVER_ERROR),
    }
}
