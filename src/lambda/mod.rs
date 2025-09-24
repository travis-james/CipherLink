use lambda_http::{run, service_fn, Request};
use crate::lambda::routing::router;
use crate::app_config::AppConfig;
use crate::db;

mod helpers;
mod routing;

use std::sync::Arc;

pub async fn init(config: AppConfig) {
    // arc allows db_client to be cloned and shared across requests.
    let db_client = Arc::new(
        db::init(&config.db_url, &config.region)
            .await
    );

    // move allows the below closure to own db_client.
    let handler = service_fn(move |event: Request| {
        let db = db_client.clone(); // Each request get it's own db_client reference.
        async move { router(event, &db).await }
    });

    run(handler).await.expect("Lambda runtime failed");
}
