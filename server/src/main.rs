mod app_state;
mod config;
mod handler;
mod internal_store;
mod menu;
mod order;
mod table;

use crate::app_state::AppState;
use crate::config::handler::get_configs;
use crate::handler::health_check_handler;
use crate::menu::handler::get_available_menus;
use crate::order::handler::create_orders;
use axum::http::{HeaderValue, Method};
use axum::routing::{delete, get, post};
use axum::Router;
use dotenv::dotenv;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use crate::table::handler::{delete_table_order, get_table_order, get_table_orders};

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

    let app_state = Arc::new(AppState::new());

    let table_routes = Router::new()
        .route("/:id/orders", get(get_table_orders))
        .route("/:id/orders/:order_id", get(get_table_order))
        .route("/:id/orders/:order_id", delete(delete_table_order));

    let app = Router::new()
        .route("/health", get(health_check_handler))
        .route("/configs", get(get_configs))
        .route("/menus", get(get_available_menus))
        .route("/orders", post(create_orders))
        .nest("/tables", table_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(cors)
        // Reference to app state and potentially put database connection pool here
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind(server_address.clone())
        .await
        .unwrap();
    info!("Server started successfully");
    axum::serve(listener, app).await.unwrap();
}
