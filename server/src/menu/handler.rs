use crate::internal_store::MENUS;
use axum::response::IntoResponse;
use axum::Json;
use std::ops::Deref;
use std::sync::Arc;
use tracing::info;

pub async fn get_available_menus() -> impl IntoResponse {
    let menus = Arc::clone(&MENUS);
    let data = menus.deref();
    let json_response = serde_json::json!({
        "status": "success",
        "data": *data
    });
    info!("Available menus: {:?}", json_response);
    Json(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::model::MenuData;
    use axum::body::to_bytes;
    use axum::response::Response;
    use http::StatusCode;
    use serde::Deserialize;
    use serde_json::Value;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_available_menus() {
        let response: Response = get_available_menus().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "success");
        assert!(json["data"].is_array());

        let menus = json["data"].as_array().unwrap();
        assert!(!menus.is_empty()); // Should have at least one menu

        let first_menu = &menus[0];
        assert!(first_menu["id"].is_string());
        assert!(first_menu["name"].is_string());

        let menu_names: Vec<&str> = menus.iter().map(|m| m["name"].as_str().unwrap()).collect();
        assert!(menu_names.contains(&"Ramen"));
        assert!(menu_names.contains(&"Beer"));
        assert!(menu_names.contains(&"Beef rice"));
    }

    #[tokio::test]
    async fn test_menu_uniqueness() {
        let response: Response = get_available_menus().await.into_response();
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        let menus = json["data"].as_array().unwrap();

        let mut ids: Vec<&str> = menus.iter().map(|m| m["id"].as_str().unwrap()).collect();
        let original_len = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), original_len, "All menu IDs should be unique");

        let mut names: Vec<&str> = menus.iter().map(|m| m["name"].as_str().unwrap()).collect();
        let original_len = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), original_len, "All menu names should be unique");
    }

    #[tokio::test]
    async fn test_menu_response_serialization() {
        let response: Response = get_available_menus().await.into_response();
        let body = to_bytes(response.into_body(), 1024).await.unwrap();

        #[derive(Debug, Deserialize)]
        struct ApiResponse {
            status: String,
            data: Vec<MenuData>,
        }

        let api_response: ApiResponse = serde_json::from_slice(&body).unwrap();
        assert!(!api_response.data.is_empty());
        assert!(api_response.status.eq("success"));
    }

    #[tokio::test]
    async fn test_menu_content_validation() {
        let response: Response = get_available_menus().await.into_response();
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        let menus = json["data"].as_array().unwrap();

        for menu in menus {
            let id = menu["id"].as_str().unwrap();
            assert!(Uuid::parse_str(id).is_ok(), "Menu ID should be valid UUID");

            let name = menu["name"].as_str().unwrap();
            assert!(!name.is_empty(), "Menu name should not be empty");
            assert!(name.len() <= 50, "Menu name should not be too long");
        }
    }
}
