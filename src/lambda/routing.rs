use http::StatusCode;
use lambda_http::{Body, Request, Response};
use lambda_runtime::Error;

use crate::{
    db::DynamoDBClient,
    handlers::{decrypt_handler, encrypt_handler, health_handler},
    lambda::helpers::{error_payload, json_response, redirect_response},
    types::{EncryptRequest, HealthStatus},
};

pub async fn router(
    event: Request,
    db_client: &DynamoDBClient,
) -> Result<Response<lambda_http::Body>, Error> {
    let path = event.uri().path();
    let method = event.method().as_str();

    let resp = match (method, path) {
        ("GET", "/health") => lambda_health_handler().await,
        ("POST", "/encrypt") => lambda_encrypt_handler(event, db_client).await,
        _ if path.starts_with("/decrypt/") => lambda_decrypt_handler(path, db_client).await,
        _ => crate::lambda::helpers::json_response(
            &crate::lambda::helpers::error_payload("Not Found"),
            http::StatusCode::NOT_FOUND,
        ),
    };

    Ok(resp)
}

pub async fn lambda_health_handler() -> Response<Body> {
    let status: HealthStatus = health_handler().await;
    let body = serde_json::to_string(&status).unwrap_or_else(|_| "{}".to_string());

    json_response(&body, StatusCode::OK)
}

pub async fn lambda_encrypt_handler(event: Request, db_client: &DynamoDBClient) -> Response<Body> {
    let body_string = match event.body() {
        Body::Text(s) => s.clone(),
        Body::Binary(b) => match String::from_utf8(b.clone()) {
            Ok(s) => s,
            Err(_) => {
                return json_response(&error_payload("Invalid UTF-8"), StatusCode::BAD_REQUEST);
            }
        },
        Body::Empty => return json_response(&error_payload("Empty body"), StatusCode::BAD_REQUEST),
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

pub async fn lambda_decrypt_handler(path: &str, db_client: &DynamoDBClient) -> Response<Body> {
    let parts: Vec<&str> = path.trim_start_matches("/decrypt/").split('/').collect();
    if parts.len() != 2 {
        return json_response(
            &error_payload("Invalid decrypt path"),
            StatusCode::BAD_REQUEST,
        );
    }

    let decrypted = decrypt_handler(db_client, parts[0].to_string(), parts[1].to_string()).await;
    if let Ok(url) = decrypted {
        if let Ok(valid_url) = url::Url::parse(&url) {
            return redirect_response(valid_url.as_str());
        } else {
            return json_response(&error_payload("Invalid URL"), StatusCode::BAD_REQUEST);
        }
    } else {
        return json_response(&decrypted.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
