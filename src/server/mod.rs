use axum::{
    response::IntoResponse, routing::get, Router
};

pub async fn init() {
    // build our application with a single route
    let app = Router::new().route("/health", get(health));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> impl IntoResponse {
    "healthy"
}