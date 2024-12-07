use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "OK" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}