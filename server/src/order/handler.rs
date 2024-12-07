use crate::order::model::Order;
use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type OrderStorage = Arc<RwLock<HashMap<u32, Vec<Order>>>>;

static ORDERS: Lazy<OrderStorage> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

fn random_cooking_time() -> u32 {
    rand::rng().random_range(5..=15)
}