use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
struct FileConfig {
    api_key: Option<String>,
    environment: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub environment: String,
}

impl Config {
    pub fn load(config_path: Option<&Path>) -> Result<Self, String> {
        // Try loading from explicit path, or default ~/.billplz/config.toml
        let file_config = match config_path {
            Some(p) => std::fs::read_to_string(p).ok(),
            None => dirs::home_dir()
                .map(|h| h.join(".billplz").join("config.toml"))
                .and_then(|p| std::fs::read_to_string(p).ok()),
        }
        .and_then(|s| toml::from_str::<FileConfig>(&s).ok())
        .unwrap_or_default();

        // Env vars override file config
        let api_key = std::env::var("BILLPLZ_API_KEY")
            .ok()
            .or(file_config.api_key)
            .ok_or("API key not found. Set BILLPLZ_API_KEY env var or add api_key to ~/.billplz/config.toml")?;

        let environment = std::env::var("BILLPLZ_ENVIRONMENT")
            .ok()
            .or(file_config.environment)
            .unwrap_or_else(|| "staging".to_string());

        Ok(Config { api_key, environment })
    }

    pub fn into_client(self) -> crate::BillplzClient {
        let env = match self.environment.as_str() {
            "production" => crate::Environment::Production,
            _ => crate::Environment::Staging,
        };
        crate::BillplzClient::new(env, self.api_key)
    }
}
