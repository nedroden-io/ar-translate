use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub azure_api_key: String,
    pub azure_api_region: String,
    pub azure_api_endpoint: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let api_key = std::env::var("AR_TRANSLATE_AZURE_API_KEY").context(
            "Failed to load Azure API key from environment variable 'AR_TRANSLATE_AZURE_API_KEY'",
        )?;

        let api_region =
            std::env::var("AR_TRANSLATE_AZURE_API_REGION").unwrap_or("westeurope".to_string());

        let api_endpoint = std::env::var("AR_TRANSLATE_AZURE_API_ENDPOINT")
            .context("Failed to load Azure API key from environment variable 'AR_TRANSLATE_AZURE_API_ENDPOINT'")?;

        Ok(Self {
            azure_api_key: api_key,
            azure_api_region: api_region,
            azure_api_endpoint: api_endpoint,
        })
    }
}
