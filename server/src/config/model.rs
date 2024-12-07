use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub table_range: (u32, u32),
}

#[derive(Error, Debug, Clone)]
pub enum ConfigError {
    #[error("Start must be greater than 0")]
    InvalidStart,
    #[error("End must be greater than or equal to start")]
    InvalidRange,
}

impl Config {
    pub fn new(table_range: (u32, u32)) -> Result<Self, ConfigError> {
        if table_range.0 == 0 {
            return Err(ConfigError::InvalidStart);
        }
        if table_range.0 > table_range.1 {
            return Err(ConfigError::InvalidRange);
        }
        Ok(Self { table_range })
    }
}
