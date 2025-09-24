use crate::app_config::AppConfig;
use crate::db;
use crate::lambda::routing::router;
use lambda_http::{Request, run, service_fn};

mod helpers;
mod routing;

use std::sync::Arc;

/// Start lambda rust runtime.
pub async fn init(config: AppConfig) {
    // arc allows db_client to be cloned and shared across requests.
    let db_client = Arc::new(db::init(&config.db_url, &config.region).await);

    // move allows the below closure to own db_client.
    let handler = service_fn(move |event: Request| {
        let db = db_client.clone(); // Each request get it's own db_client reference.
        async move { router(event, &db).await }
    });

    run(handler).await.expect("Lambda runtime failed");
}
