use http::StatusCode;
use lambda_http::{Body, Response};
use serde_json::json;

pub fn json_response<T: serde::Serialize>(data: &T, status_code: StatusCode) -> Response<Body> {
    let body = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());
    Response::builder()
        .status(status_code)
        .header("content-type", "application/json")
        .body(body.into())
        .unwrap()
}

pub fn redirect_response(redirect_url: &str) -> Response<Body> {
    let header_key = "location";
    build_response(StatusCode::FOUND, header_key, redirect_url, "")
}

pub fn error_payload(msg: &str) -> serde_json::Value {
    json!({ "error": msg })
}

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
