use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::menu::model::Menu;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub table_id: u32,
    pub menu: Menu,
    pub cooking_time_minutes: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub table_id: u32,
    pub menus: Vec<Menu>,
}
