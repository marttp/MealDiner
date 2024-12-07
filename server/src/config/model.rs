use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub table_range: (u32, u32),
}

impl Config {
    pub fn new(table_range: (u32, u32)) -> Self {
        Self { table_range }
    }
}
