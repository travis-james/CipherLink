use http::StatusCode;
use lambda_http::{Body, Response};
use serde_json::json;

/// Build a JSON response for lambda.
pub fn json_response<T: serde::Serialize>(data: &T, status_code: StatusCode) -> Response<Body> {
    let body = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());
    let header_key = "content-type";
    let header_val = "application/json";
    build_response(status_code, header_key, header_val, &body)
}

/// Build a redirect response for lambda.
pub fn redirect_response(redirect_url: &str) -> Response<Body> {
    let header_key = "location";
    build_response(StatusCode::FOUND, header_key, redirect_url, "")
}

/// Convert string to a json error.
pub fn error_payload(msg: &str) -> serde_json::Value {
    json!({ "error": msg })
}

/// Build a response for lambda.
pub fn build_response(
    status_code: StatusCode,
    header_key: &str,
    header_val: &str,
    body: &str,
) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .header(header_key, header_val)
        .body(body.into())
        .unwrap()
}
