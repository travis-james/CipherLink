use lambda_http::{lambda_runtime, run, service_fn, Request};
use lambda_runtime::Error;
use crate::lambda::routing::router;
use crate::app_config::AppConfig;
use crate::db;

mod helpers;
mod routing;

use std::sync::Arc;

pub async fn init(config: AppConfig) {
    let db_client = Arc::new(
        db::init(&config.db_url, &config.region)
            .await
    );

    let handler = service_fn(move |event: Request| {
        let db = db_client.clone();
        async move { router(event, &db).await }
    });

    run(handler).await.expect("Lambda runtime failed");
}
