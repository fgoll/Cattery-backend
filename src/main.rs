use axum::{
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::info;

mod api;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = db::establish_connection().await;

    // build our application with a route
    let app = Router::new()
        .route("/api/wx_counter/login", post(api::user::login))
        .route("/api/admin/admin_register", post(api::user::admin_register))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("service listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
