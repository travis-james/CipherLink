use crate::{
    crypto::{decrypt, encrypt},
    db::DynamoDBClient,
    transformer::{encrypt_data_to_item, item_to_encryt_data},
    types::{EncryptRequest, EncryptResponse, HealthStatus},
};

/// health_handler is just used to see if one can get a response
/// from the app.
pub async fn health_handler() -> HealthStatus {
    HealthStatus { status: "healthy" }
}

/// encrypt_handler for the /encrypt endpoint.
/// expects a POST and json body like:
/// {"plain_text":"http://yahoo.com","key":"foobar"}
/// Returns a UUID that needeed for decryption.
///
/// # Errors
/// Encryption and inserting to the db can fail.
pub async fn encrypt_handler(
    db_client: &DynamoDBClient,
    payload: EncryptRequest,
) -> Result<EncryptResponse, String> {
    let encrypted_data =
        encrypt(&payload.plain_text, &payload.key).map_err(|_| "Encryption failed")?;

    let id = uuid::Uuid::new_v4().to_string();
    let item = encrypt_data_to_item(&id, &encrypted_data);

    db_client
        .insert("encryptData", item)
        .await
        .map_err(|e| format!("DB insert failed: {}", e))?;

    Ok(EncryptResponse { id })
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
pub async fn decrypt_handler(
    db_client: &DynamoDBClient,
    id: String,
    key: String,
) -> Result<String, String> {
    let data = db_client
        .get("encryptData", "id", &id)
        .await
        .map_err(|e| format!("DB get failed: {}", e))?;

    let transformed_data =
        item_to_encryt_data(&data).map_err(|e| format!("Transform failed: {}", e))?;

    let decrypted_data =
        decrypt(&transformed_data, &key).map_err(|e| format!("Decrypt failed: {}", e))?;

    db_client
        .delete("encryptData", "id", &id)
        .await
        .map_err(|e| format!("Delete failed: {}", e))?;

    Ok(String::from_utf8_lossy(&decrypted_data).to_string())
}
