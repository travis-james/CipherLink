use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: &'static str,
}

#[derive(Deserialize)]
pub struct EncryptRequest {
    pub plain_text: String,
    pub key: String,
}

#[derive(Serialize)]
pub struct EncryptResponse {
    pub id: String,
}

#[derive(Serialize)]
#[serde(tag = "status", content = "data")]
pub enum EncryptApiResponse {
    Ok(EncryptResponse),
    Err(String),
}

#[derive(Debug, Deserialize)]
pub struct DecryptParams {
    pub id: String,
    pub key: String,
}
