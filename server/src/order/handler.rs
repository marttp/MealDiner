use crate::app_state::AppState;
use crate::config::handler::get_config_internally;
use crate::order::model::{CreateOrderRequest, Order};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use rand::Rng;
use serde_json::json;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub async fn create_orders(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let config = get_config_internally();

    if !(config.table_range.0..=config.table_range.1).contains(&payload.table_id) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut new_orders = Vec::new();
    let now = Utc::now();

    // Iterate through each input menus and treat them individually into order
    for menu in payload.menus {
        let order = Order {
            id: Uuid::new_v4(),
            table_id: payload.table_id,
            menu,
            cooking_time_minutes: random_cooking_time(),
            created_at: now,
        };
        new_orders.push(order);
    }

    let mut orders = state.orders.write().await;
    orders
        .entry(payload.table_id)
        .or_default()
        .extend(new_orders.clone());

    info!("Created orders for table: {:?}", payload.table_id);

    let response = Json(json!({
        "status": "success",
        "data": new_orders
    }));

    Ok(response)
}

fn random_cooking_time() -> u32 {
    rand::rng().random_range(5..=15)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::model::MenuData;

    fn create_test_state() -> Arc<AppState> {
        Arc::new(AppState::new())
    }

    fn create_test_menu() -> MenuData {
        MenuData {
            id: Uuid::new_v4(),
            name: "Test Menu".to_string(),
        }
    }

    #[tokio::test]
    async fn test_create_orders_success() {
        let state = create_test_state();
        let menu = create_test_menu();

        let payload = CreateOrderRequest {
            table_id: 1,
            menus: vec![menu.clone()],
        };

        let result = create_orders(
            State(state.clone()),
            Json(payload)
        ).await;

        match result {
            Ok(response) => {
                let response = response.into_response();
                assert_eq!(response.status(), StatusCode::OK);

                let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
                let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

                assert_eq!(json["status"], "success");
                let data = json["data"].as_array().unwrap();
                assert_eq!(data.len(), 1);
                assert_eq!(data[0]["menu"]["name"], menu.name);

                let orders = state.orders.read().await;
                assert_eq!(orders.get(&1).unwrap().len(), 1);
            },
            Err(_) => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_create_orders_invalid_table() {
        let state = create_test_state();
        let payload = CreateOrderRequest {
            table_id: 99999,  // Invalid table number
            menus: vec![create_test_menu()],
        };

        let result = create_orders(
            State(state),
            Json(payload)
        ).await;

        match result {
            Ok(_) => panic!("Expected error response"),
            Err(status) => assert_eq!(status, StatusCode::BAD_REQUEST),
        }
    }

    #[tokio::test]
    async fn test_create_multiple_orders() {
        let state = create_test_state();
        let menus = vec![create_test_menu(), create_test_menu(), create_test_menu()];

        let payload = CreateOrderRequest {
            table_id: 1,
            menus: menus.clone(),
        };

        let result = create_orders(
            State(state.clone()),
            Json(payload)
        ).await;

        match result {
            Ok(response) => {
                let response = response.into_response();
                assert_eq!(response.status(), StatusCode::OK);

                let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
                let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

                let data = json["data"].as_array().unwrap();
                assert_eq!(data.len(), 3);

                let orders = state.orders.read().await;
                assert_eq!(orders.get(&1).unwrap().len(), 3);
            },
            Err(_) => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_order_timestamps() {
        let state = create_test_state();
        let menu = create_test_menu();

        let payload = CreateOrderRequest {
            table_id: 1,
            menus: vec![menu],
        };

        let before = Utc::now();
        let result = create_orders(
            State(state),
            Json(payload)
        ).await;
        let after = Utc::now();

        match result {
            Ok(response) => {
                let response = response.into_response();
                let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
                let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

                let created_at = chrono::DateTime::parse_from_rfc3339(
                    json["data"][0]["created_at"].as_str().unwrap()
                ).unwrap();

                assert!(created_at >= before && created_at <= after);
            },
            Err(_) => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_create_orders_empty_menu_list() {
        let state = create_test_state();
        let payload = CreateOrderRequest {
            table_id: 1,
            menus: vec![],
        };

        let result = create_orders(
            State(state.clone()),
            Json(payload)
        ).await;

        match result {
            Ok(response) => {
                let response = response.into_response();
                assert_eq!(response.status(), StatusCode::OK);

                let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
                let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
                assert!(json["data"].as_array().unwrap().is_empty());

                let orders = state.orders.read().await;
                assert!(orders.get(&1).unwrap_or(&vec![]).is_empty());
            },
            Err(_) => panic!("Expected success response"),
        }
    }

    #[test]
    fn test_random_cooking_time() {
        for _ in 0..100 {
            let cooking_time = random_cooking_time();
            assert!(cooking_time >= 5 && cooking_time <= 15);
        }
    }
}