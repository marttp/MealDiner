use axum::Json;
use axum::response::IntoResponse;
use crate::config::model::Config;

pub fn get_config_internally() -> Config {
    let max_tables = std::env::var("AVAILABLE_TABLES").unwrap_or("10000".to_string());
    let config = Config::new((1, max_tables.parse::<u32>().unwrap()));
    config
}

pub async fn get_configs() -> impl IntoResponse {
    let config = get_config_internally();
    let json_response = serde_json::json!({
        "status": "success",
        "data": config
    });
    Json(json_response)
}
