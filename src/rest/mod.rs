use axum::{
    Extension, Json, Router,
    extract::Path,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};

use crate::{
    app_config::AppConfig,
    db::{self, DynamoDBClient},
    handlers::{decrypt_handler, encrypt_handler, health_handler},
    types::{DecryptParams, EncryptApiResponse, EncryptRequest},
};

///  Initialize the app. Creates and runs an axum server and a
/// dynamodb client based on the input config.
pub async fn init(config: AppConfig) {
    let db_client = db::init(&config.db_url, &config.region).await;

    let app = Router::new()
        .route("/health", get(rest_health_handler))
        .route("/encrypt", post(rest_encrypt_handler))
        .route("/decrypt/{id}/{key}", get(rest_decrypt_handler))
        .layer(Extension(db_client));

    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// health_handler is just used to see if one can get a response
/// from the app.
async fn rest_health_handler() -> Response {
    let status = health_handler().await;
    Json(status).into_response()
}

/// encrypt_handler for the /encrypt endpoint.
/// expects a POST and json body like:
/// {"plain_text":"http://yahoo.com","key":"foobar"}
/// Returns a UUID that needeed for decryption.
///
/// # Errors
/// Encryption and inserting to the db can fail.
pub async fn rest_encrypt_handler(
    Extension(db_client): Extension<DynamoDBClient>,
    Json(payload): Json<EncryptRequest>,
) -> Response {
    match encrypt_handler(&db_client, payload).await {
        Ok(resp) => Json(EncryptApiResponse::Ok(resp)).into_response(),
        Err(err) => Json(EncryptApiResponse::Err(err)).into_response(),
    }
}

/// decrypt_handler is used for the /decrypt/{id}/{key} endpoint.
/// Requires the key used for the original decryption and UUID
/// that was returned when the encrypt handle was called.
/// Assuming a valid UUID and key, the app will redirect the user
/// to the encrypted URL. The database entry is then deleted.
///
/// # Errors
/// Potential failures on the following steps retrieving/deleting
/// from the db, decoding/transforming the data from the db,
/// and decryption.
async fn rest_decrypt_handler(
    Extension(db_client): Extension<DynamoDBClient>,
    Path(params): Path<DecryptParams>,
) -> Response {
    match decrypt_handler(&db_client, params.id, params.key).await {
        Ok(url) => match url::Url::parse(&url) {
            Ok(valid_url) => Redirect::temporary(valid_url.as_str()).into_response(),
            Err(_) => "Decrypted data is not a valid URL".into_response(),
        },
        Err(err) => Json(err).into_response(),
    }
}
