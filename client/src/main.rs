mod model;
mod restaurant_client;

use crate::restaurant_client::RestaurantClient;
use dotenv::dotenv;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let base_url = std::env::var("SERVER_HOST").expect("SERVER_HOST is not set");

    // Refer to each device, not table
    let mut client = RestaurantClient::new(base_url.clone());
    client.initialize().await?;

    println!("{:?}", client);

    Ok(())
}