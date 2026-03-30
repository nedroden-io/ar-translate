use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    azure_api_key: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let api_key = std::env::var("AZURE_API_KEY")
            .context("Failed to load Azure API key from environment variable 'AZURE_API_KEY'")?;

        Ok(Self { azure_api_key: api_key })
    }
}