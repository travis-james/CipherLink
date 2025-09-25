use http::StatusCode;
use lambda_http::{Body, Response};
use serde_json::json;

/// Build a JSON response for lambda.
pub fn json_response<T: serde::Serialize>(data: &T, status_code: StatusCode) -> Response<Body> {
    let json = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());
    Response::builder()
        .status(status_code)
        .header("content-type", "application/json")
        .body(Body::Text(json))
        .unwrap()
}

/// Build a redirect response for lambda.
pub fn redirect_response(redirect_url: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FOUND)
        .header("location", redirect_url)
        .body(Body::Empty)
        .unwrap()
}

/// Convert string to a json error.
pub fn error_payload(msg: &str) -> serde_json::Value {
    json!({ "error": msg })
}

/// Extracts what is expected to be a string from the body of a
/// lambda event. It's not reused, just wanted to lighten the
/// coginitive load in lambda/routing.rs.
pub fn extract_body_string(body: &Body) -> Result<String, Response<Body>> {
    match body {
        Body::Text(s) => Ok(s.clone()),
        Body::Binary(b) => String::from_utf8(b.clone())
            .map_err(|_| json_response(&error_payload("Invalid UTF-8"), StatusCode::BAD_REQUEST)),
        Body::Empty => Err(json_response(
            &error_payload("Empty body"),
            StatusCode::BAD_REQUEST,
        )),
    }
}
