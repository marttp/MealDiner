use crate::config::model::Config;
use axum::response::IntoResponse;
use axum::Json;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_internally_default() {
        std::env::remove_var("AVAILABLE_TABLES");

        let config = get_config_internally();
        assert_eq!(config.table_range.0, 1);
        assert_eq!(config.table_range.1, 10000);
    }

    #[test]
    fn test_get_config_internally_with_env() {
        std::env::set_var("AVAILABLE_TABLES", "5000");

        let config = get_config_internally();
        assert_eq!(config.table_range.0, 1);
        assert_eq!(config.table_range.1, 5000);

        std::env::remove_var("AVAILABLE_TABLES");
    }

    #[test]
    fn test_config_new() {
        let config = Config::new((1, 100));
        assert_eq!(config.table_range.0, 1);
        assert_eq!(config.table_range.1, 100);
    }
}