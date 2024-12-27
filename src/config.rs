use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Deserialize)]
pub struct Config {
    pub url: String,
    pub timeout: u64,
}

pub fn load_config() -> Result<Config> {
    let contents = fs::read_to_string("config.toml")
    .expect("Could not read configuration file");
    let config: Config = toml::from_str(&contents)
    .expect("Failed to parse configuratin file");
    Ok(config)
}

