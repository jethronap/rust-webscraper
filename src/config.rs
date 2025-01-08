use serde::Deserialize;
use std::fs;
use anyhow::{Context, Result};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub url: Option<String>,
    pub timeout: Option<u64>,
}

pub fn load_config(file_path: &str) -> Result<Config> {
    let contents = fs::read_to_string(file_path)
    .with_context(|| format!("Could not read configuration file: {}", file_path))?;
    let config: Config = toml::from_str(&contents)
    .with_context(|| "Failed to parse configuratin file")?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_config_valid() {
        // Create a temporary configuration file
        let config_content = r#"
            url = "https://www.rust-lang.org"
            timeout = 30
            "#;
        fs::write("test_config.toml", config_content).unwrap();

        // Load the configuration file and validate
        let config = load_config("test_config.toml").unwrap();
        assert_eq!(config.url.unwrap(), "https://www.rust-lang.org");
        assert_eq!(config.timeout.unwrap(), 30);

        // Clean up
        fs::remove_file("test_config.toml").unwrap();
    }

    #[test]
    fn test_load_config_missing_file() {
        // Ensure no file exists
        fs::remove_file("missing_config.toml").ok();

        // Attempt to load the configuration file and expect an error
        let result = load_config("missing_config.toml");
        assert!(result.is_err());
    }
}
