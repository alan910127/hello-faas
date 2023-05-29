use crate::prelude::*;
use crate::CONFIG_PATH;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub runtime: RuntimeConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuntimeConfig {
    #[serde(rename = "base-image")]
    pub base_image: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config = std::fs::read_to_string(CONFIG_PATH)
            .with_context(|| format!("Failed to read config file at {}", CONFIG_PATH))?;
        let config: Config = toml::from_str(&config)
            .with_context(|| format!("Failed to parse config file at {}", CONFIG_PATH))?;
        Ok(config)
    }
}
