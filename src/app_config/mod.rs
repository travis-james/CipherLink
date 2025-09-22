use serde::Deserialize;
use dotenv::dotenv;
use config as config_crate;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub region: String,
    pub db_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        config_crate::Config::builder()
            .add_source(config_crate::Environment::default().separator("_"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        unsafe { std::env::set_var("CONFIG_REGION", "us-west-2") };
        unsafe { std::env::set_var("CONFIG_DB_URL", "http://localhost:8000") };
        unsafe { std::env::set_var("CONFIG_SERVER_PORT", "3000") };

        let config = Config::from_env();
        assert_eq!(config.region, "us-west-2");
        assert_eq!(config.db_url, "http://localhost:8000");
        assert_eq!(config.server_port, 3000);
    }
}