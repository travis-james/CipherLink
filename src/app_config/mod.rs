use std::env;

/// Configuration values loaded from environment variables.
///
/// Holds runtime configuration for the application.
pub struct AppConfig {
    pub region: String,
    pub db_url: String,
    pub server_port: u16,
}

impl AppConfig {
    /// Constructs an `AppConfig` from environment variables.
    ///
    /// # Panics
    /// Panics if any required environment variable is missing or malformed.
    pub fn from_env() -> Self {
        let region = env::var("CONFIG_REGION").expect("unable to get CONFIG_REGION");
        let db_url = env::var("CONFIG_DB_URL").expect("unable to get CONFIG_DB_URL");
        let server_port = env::var("CONFIG_SERVER_PORT")
            .expect("unable to get CONFIG_SERVER_PORT")
            .parse::<u16>()
            .expect("CONFIG_SERVER_PORT must be a valid u16");

        AppConfig {
            region,
            db_url,
            server_port,
        }
    }
}
