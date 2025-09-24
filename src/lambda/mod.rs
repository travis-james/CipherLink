use lambda_http::{handler, lambda_runtime, Request};
use lambda_runtime::Error;
use crate::lambda::routing::router;
use crate::app_config::AppConfig;
use crate::db;

mod helpers;
mod routing;

pub async fn serve(config: AppConfig) -> Result<(), Error> {
    let db_client = db::init(&config.db_url, &config.region).await;
    lambda_runtime::run(handler(move |event: Request| {
        let db_client = db_client.clone();
        async move { router(event, &db_client).await }
    }))
    .await?;
    Ok(())
}