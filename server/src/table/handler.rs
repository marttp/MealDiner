use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_table_orders(
    State(state): State<Arc<AppState>>,
    Path(table_id): Path<u32>,
) -> impl IntoResponse {
    let orders = state.orders.read().await;
    let table_orders = orders.get(&table_id).cloned().unwrap_or_default();

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
            if let Some(order) = table_orders.iter().find(|order| order.id == *order_id) {
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
    Ok(json!({ "status": "success" }))
}
