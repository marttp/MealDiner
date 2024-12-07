use crate::model::{ApiResponse, Config, MenuData, Order};
use rand::Rng;
use reqwest::Client;
use std::error::Error;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct RestaurantClient {
    client: Client,
    base_url: String,
    config: Option<Config>,
    available_menus: Vec<MenuData>,
}

impl RestaurantClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            config: None,
            available_menus: Vec::new(),
        }
    }

    pub fn get_config(&self) -> Option<Config> {
        self.config.clone()
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let config: ApiResponse<Config> = self
            .client
            .get(&format!("{}/configs", self.base_url))
            .send()
            .await?
            .json()
            .await?;
        self.config = Some(config.data);

        let menus: ApiResponse<Vec<MenuData>> = self
            .client
            .get(&format!("{}/menus", self.base_url))
            .send()
            .await?
            .json()
            .await?;
        self.available_menus = menus.data;

        println!("Get config: {:?}", self.config);
        println!("Available menus: {:?}", self.available_menus);

        Ok(())
    }

    pub async fn get_table_orders(&self, table_id: u32) -> Result<Vec<Order>> {
        let path = format!(
            "{}/tables/{}/orders",
            self.base_url, table_id
        );
        println!("Getting table orders from {}", path);
        let response: ApiResponse<Vec<Order>> = self
            .client
            .get(&path)
            .send()
            .await?
            .json()
            .await?;
        Ok(response.data)
    }

    pub async fn create_order(&self, table_id: u32, menu_count: usize) -> Result<()> {
        let selected_menus: Vec<MenuData> = {
            let mut rng = rand::rng();
            (0..menu_count)
                .map(|_| self.available_menus[rng.random_range(0..self.available_menus.len())].clone())
                .collect()
        };
        println!("Creating order for table {}", table_id);
        let payload = serde_json::json!({
            "table_id": table_id,
            "menus": selected_menus
        });
        let path = format!("{}/orders", self.base_url);
        self.client
            .post(&path)
            .json(&payload)
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete_order(&self, table_id: u32, order_id: Uuid) -> Result<()> {
        let path = format!("{}/tables/{}/orders/{}", self.base_url, table_id, order_id);
        println!("Deleting order {} for table {}", order_id, table_id);
        self.client
            .delete(&path)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_specific_order(&self, table_id: u32, order_id: Uuid) -> Result<Order> {
        let path = format!("{}/tables/{}/orders/{}", self.base_url, table_id, order_id);
        println!("Getting specific order {} for table {}", order_id, table_id);
        let response: ApiResponse<Order> = self
            .client
            .get(&path)
            .send()
            .await?
            .json()
            .await?;
        Ok(response.data)
    }
}
