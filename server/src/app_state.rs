use crate::order::model::Order;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type OrderStorage = Arc<RwLock<HashMap<u32, Vec<Order>>>>;

#[derive(Clone)]
pub struct AppState {
    pub orders: OrderStorage,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
