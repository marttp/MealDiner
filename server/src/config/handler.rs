use axum::Json;
use axum::response::IntoResponse;
use crate::config::model::Config;

pub async fn get_configs() -> impl IntoResponse {
    let max_tables = std::env::var("AVAILABLE_TABLES").unwrap_or("10000".to_string());
    let config = Config::new((1, max_tables.parse::<u32>().unwrap()));
    let json_response = serde_json::json!({
        "status": "success",
        "data": config
    });
    Json(json_response)
}
