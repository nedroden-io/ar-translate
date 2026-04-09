use crate::app_settings::AppConfig;
use anyhow::{anyhow, Result};

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

    pub async fn send_request<T: serde::de::DeserializeOwned>(
        &self,
        url: impl Into<String>,
        body: impl Into<String>,
    ) -> Result<T> {
        let response = self
            .http_client
            .post(url.into())
            .header("Ocp-Apim-Subscription-Key", &self.api_key)
            .header("Ocp-Apim-Subscription-Region", &self.api_region)
            .header("Content-Type", "application/json")
            .body(body.into())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Azure API request failed with status: {}", response.status()));
        }

        Ok(response.json::<T>().await?)
    }
}
