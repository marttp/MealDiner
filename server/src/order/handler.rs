use crate::order::model::{CreateOrderRequest, Order};
use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::config::handler::get_config_internally;

type OrderStorage = Arc<RwLock<HashMap<u32, Vec<Order>>>>;

static ORDERS: Lazy<OrderStorage> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

pub async fn create_orders(
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

    let mut orders = ORDERS.write().await;
    orders
        .entry(payload.table_id)
        .or_default()
        .extend(new_orders.clone());

    let response = Json(json!({
        "status": "success",
        "data": new_orders
    }));

    Ok(response)
}

fn random_cooking_time() -> u32 {
    rand::rng().random_range(5..=15)
}