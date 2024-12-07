mod model;
mod restaurant_client;

use crate::restaurant_client::RestaurantClient;
use dotenv::dotenv;
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use crate::model::Order;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let base_url = std::env::var("SERVER_HOST").expect("SERVER_HOST is not set");

    let mut client = RestaurantClient::new(base_url.clone());
    client.initialize().await?;

    let config = client.get_config().expect("Config not initialized");
    let (start_range, end_range) = config.table_range;
    let interval_secs = 10;
    let max_rps = 10;

    loop {
        for _ in 0..max_rps {
            let table_id = rng().random_range(start_range..=end_range);
            let client_clone = client.clone();

            tokio::spawn(async move {
                match client_clone.get_table_orders(table_id).await {
                    Ok(orders) => {
                        if orders.is_empty() {
                            handle_empty_order(table_id, &client_clone).await;
                        } else {
                            specific_operation(table_id, client_clone, orders).await;
                        }
                    }
                    Err(e) => eprintln!("Error getting orders for table {}: {}", table_id, e),
                }
            });
        }

        sleep(Duration::from_secs(interval_secs)).await;
    }
}

async fn handle_empty_order(table_id: u32, client_clone: &RestaurantClient) {
    println!("No orders found for table id {}", table_id);
    let menu_count = rng().random_range(1..=3);
    if let Err(e) = client_clone.create_order(table_id, menu_count).await {
        eprintln!("Error creating order for table {}: {}", table_id, e);
    }
}

async fn specific_operation(table_id: u32, client_clone: RestaurantClient, orders: Vec<Order>) {
    let action = rng().random_range(0..3);
    let result = match action {
        0 => {
            let menu_count = rng().random_range(1..=2);
            client_clone.create_order(table_id, menu_count).await
        }
        1 => {
            if !orders.is_empty() {
                let idx = rng().random_range(0..orders.len());
                client_clone.delete_order(table_id, orders[idx].id).await
            } else {
                Ok(())
            }
        }
        _ => {
            if !orders.is_empty() {
                let idx = rng().random_range(0..orders.len());
                client_clone
                    .get_specific_order(table_id, orders[idx].id)
                    .await
                    .map(|_| ())
            } else {
                Ok(())
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error for table {}: {}", table_id, e);
    }
}