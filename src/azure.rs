use crate::app_settings::AppConfig;
use anyhow::Result;

pub struct AzureClient {
    api_key: String,
    api_region: String,
    api_endpoint: String,

    http_client: reqwest::Client,
}

impl AzureClient {
    pub fn new(
        api_key: impl Into<String>,
        api_region: impl Into<String>,
        api_endpoint: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            api_region: api_region.into(),
            api_endpoint: api_endpoint.into(),
            http_client: reqwest::Client::new(),
        }
    }
}

pub async fn perform_azure_request<T: serde::de::DeserializeOwned>(
    app_config: &AppConfig,
    url: &str,
    body: &str,
) -> Result<T> {
    todo!("Implement this stuff")
}
