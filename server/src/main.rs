mod handler;

use axum::http::{HeaderValue, Method};
use axum::routing::get;
use axum::Router;
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use tracing::info;
use crate::handler::health_check_handler;

#[tokio::main]
async fn main() {
    // Load env - If deploy in container solution. Env should available on injection.
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let defined_port = std::env::var("SERVER_PORT").unwrap_or("8000".to_string());
    let server_address = format!("{}:{}", "0.0.0.0", defined_port);

    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE]);

    let app = Router::new().route("/health", get(health_check_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(server_address.clone())
        .await
        .unwrap();
    info!("Server started successfully");
    axum::serve(listener, app).await.unwrap();
}
