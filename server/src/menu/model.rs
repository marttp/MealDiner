use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    pub id: Uuid,
    pub name: &'static str,
}

impl Menu {
    pub fn new(name: &'static str) -> Self {
        Menu {
            id: Uuid::new_v4(),
            name,
        }
    }
}
