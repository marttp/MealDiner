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