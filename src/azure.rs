use anyhow::Result;
use crate::app_settings::AppConfig;

pub async fn perform_azure_request<T>(app_config: &AppConfig, url: &str, body: &str) -> Result<T> {
    todo!("Implement this stuff")
}