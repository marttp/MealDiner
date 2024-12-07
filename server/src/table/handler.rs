use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

pub async fn get_table_orders(
    State(state): State<Arc<AppState>>,
    Path(table_id): Path<u32>,
) -> impl IntoResponse {
    let orders = state.orders.read().await;
    let table_orders = orders.get(&table_id).cloned().unwrap_or_default();
    debug!("get_table_order: {:?}", table_orders);
    let response = json!({
        "status": "success",
        "data": table_orders
    });
    Json(response)
}

pub async fn get_table_order(
    State(state): State<Arc<AppState>>,
    Path((table_id, order_id)): Path<(u32, Uuid)>,
) -> Result<impl IntoResponse, StatusCode> {
    let orders = state.orders.read().await;
    match orders.get(&table_id) {
        Some(table_orders) => {
            if let Some(order) = table_orders.iter().find(|order| order.id == order_id) {
                info!("get_table_order: {:?}", order);
                let response = json!({ "status": "success", "data": order });
                Ok(Json(response))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_table_order(
    State(state): State<Arc<AppState>>,
    Path((table_id, order_id)): Path<(u32, Uuid)>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut orders = state.orders.write().await;
    if let Some(table_orders) = orders.get_mut(&table_id) {
        let current_size = table_orders.len();
        table_orders.retain(|order| order.id != order_id);
        // Succeed to remove
        if table_orders.len() < current_size {
            info!("deleting order {} on table {}", current_size, table_id);
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::model::{MenuData, Order};
    use axum::response::Response;
    use chrono::Utc;

    fn create_test_state() -> Arc<AppState> {
        Arc::new(AppState::new())
    }

    fn create_test_order(table_id: u32) -> Order {
        Order {
            id: Uuid::new_v4(),
            table_id,
            menu: MenuData {
                id: Uuid::new_v4(),
                name: "Test Menu".to_string(),
            },
            cooking_time_minutes: 10,
            created_at: Utc::now(),
        }
    }

    async fn setup_test_orders(state: &AppState, table_id: u32, count: usize) -> Vec<Order> {
        let mut orders = Vec::new();
        {
            let mut state_orders = state.orders.write().await;
            for _ in 0..count {
                let order = create_test_order(table_id);
                orders.push(order.clone());
                state_orders.entry(table_id).or_default().push(order);
            }
        }
        orders
    }

    #[tokio::test]
    async fn test_delete_order_success() {
        let state = create_test_state();
        let orders = setup_test_orders(&state, 1, 1).await;
        let order_id = orders[0].id;

        let result = delete_table_order(State(state.clone()), Path((1, order_id))).await;

        match result {
            Ok(response) => {
                let response = response.into_response();
                assert_eq!(response.status(), StatusCode::NO_CONTENT);
            }
            Err(_) => panic!("Expected success response"),
        }

        let orders = state.orders.read().await;
        assert!(orders.get(&1).unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_delete_order_not_found() {
        let state = create_test_state();
        let result = delete_table_order(State(state), Path((1, Uuid::new_v4()))).await;

        match result {
            Ok(_) => panic!("Expected error response"),
            Err(status) => assert_eq!(status, StatusCode::NOT_FOUND),
        }
    }
    #[tokio::test]
    async fn test_concurrent_operations() {
        let state = create_test_state();
        let orders = setup_test_orders(&state, 1, 5).await;
        let order_id = orders[0].id;

        let mut handles = vec![];

        let state_clone = state.clone();
        handles.push(tokio::spawn(async move {
            let response: Response = get_table_orders(State(state_clone), Path(1))
                .await
                .into_response();
            assert_eq!(response.status(), StatusCode::OK);
        }));

        let state_clone = state.clone();
        handles.push(tokio::spawn(async move {
            let result = get_table_order(State(state_clone), Path((1, order_id))).await;
            match result {
                Ok(response) => {
                    let response = response.into_response();
                    assert_eq!(response.status(), StatusCode::OK);
                }
                Err(_) => panic!("Expected success response"),
            }
        }));

        let state_clone = state.clone();
        handles.push(tokio::spawn(async move {
            let result = delete_table_order(State(state_clone), Path((1, order_id))).await;
            match result {
                Ok(response) => {
                    let response = response.into_response();
                    assert_eq!(response.status(), StatusCode::NO_CONTENT);
                }
                Err(_) => panic!("Expected success response"),
            }
        }));

        for handle in handles {
            handle.await.unwrap();
        }
    }
}
