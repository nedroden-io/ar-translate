use anyhow::Result;
use crate::app_settings::AppConfig;

pub struct AzureClient {
    api_key: String,
    api_region: String,
}

impl AzureClient {
    pub fn new(api_key: impl Into<String>, api_region: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            api_region: api_region.into(),
        }
    }
}

pub async fn perform_azure_request<T>(app_config: &AppConfig, url: &str, body: &str) -> Result<T> {
    todo!("Implement this stuff")
}