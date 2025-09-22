use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub region: String,
    pub db_url: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let region = env::var("CONFIG_REGION").expect("unable to get region");
        let db_url = env::var("CONFIG_DB_URL").expect("unable to get db url");
        let server_port = env::var("CONFIG_SERVER_PORT")
            .expect("unable to get server port")
            .parse::<u16>()
            .expect("CONFIG_SERVER_PORT must be a valid u16");

        AppConfig {
            region,
            db_url,
            server_port,
        }
    }
}
