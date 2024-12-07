mod handler;

use axum::routing::get;
use axum::Router;
use dotenv::dotenv;
use crate::handler::health_check_handler;

#[tokio::main]
async fn main() {
    // Load env - If deploy in container solution. Env should available on injection.
    dotenv().ok();
    let defined_port = std::env::var("SERVER_PORT").unwrap_or("8000".to_string());
    let app = Router::new().route("/health", get(health_check_handler));
    let server_address = format!("{}:{}", "0.0.0.0", defined_port);
    let listener = tokio::net::TcpListener::bind(server_address.clone())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
