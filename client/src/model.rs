use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuData {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub table_range: (u32, u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub table_id: u32,
    pub menu: MenuData,
    pub cooking_time_minutes: u32,
    pub created_at: String,
}
