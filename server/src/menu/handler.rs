use crate::menu::model::Menu;
use axum::response::IntoResponse;
use axum::Json;
use std::ops::Deref;
use std::sync::Arc;
use once_cell::sync::Lazy;

static MENUS: Lazy<Arc<[Menu]>> = Lazy::new(|| {
    Arc::new([
        Menu::new("Ramen"),
        Menu::new("Beef rice"),
        Menu::new("Beer"),
    ])
});

pub async fn get_available_menus() -> impl IntoResponse {
    let menus = Arc::clone(&MENUS);
    let data = menus.deref();
    let json_response = serde_json::json!({
        "status": "success",
        "data": *data
    });
    Json(json_response)
}