// src/config.rs
use config::{Config as RawConfig, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub region: String,
    pub db_url: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        RawConfig::builder()
            .add_source(Environment::with_prefix("CONFIG").separator("_"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
