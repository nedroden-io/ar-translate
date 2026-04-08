use crate::azure::AzureClient;
use anyhow::Result;

pub trait MarkdownTranslator {
    fn translate_markdown(&self, input: &str, target_language: &str) -> Result<String>;
}

pub struct AzureTranslator<'a> {
    azure_client: &'a AzureClient,
}

impl<'a> AzureTranslator<'a> {
    pub fn new(azure_client: &'a AzureClient) -> AzureTranslator<'a> {
        Self { azure_client }
    }
}

impl<'a> MarkdownTranslator for AzureTranslator<'a> {
    fn translate_markdown(&self, input: &str, target_language: &str) -> Result<String> {
        let _ = input;

        Ok(format!("Translating '{input}' to '{target_language}'"))
    }
}
