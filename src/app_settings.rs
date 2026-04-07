use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    azure_api_key: String,
    azure_api_region: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let api_key = std::env::var("AR_TRANSLATE_AZURE_API_KEY")
            .context("Failed to load Azure API key from environment variable 'AR_TRANSLATE_AZURE_API_KEY'")?;

        let api_region = std::env::var("AR_TRANSLATE_AZURE_API_REGION").unwrap_or("westeurope".to_string());

        Ok(Self { azure_api_key: api_key, azure_api_region: api_region })
    }
}