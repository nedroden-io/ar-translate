use crate::azure::AzureClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub trait MarkdownTranslator {
    async fn translate_markdown(&self, input: &str, target_language: &str) -> Result<String>;
}

pub struct AzureTranslator<'a> {
    azure_client: &'a AzureClient<'a>,
}

impl<'a> AzureTranslator<'a> {
    pub fn new(azure_client: &'a AzureClient) -> AzureTranslator<'a> {
        Self { azure_client }
    }
}

impl<'a> MarkdownTranslator for AzureTranslator<'a> {
    async fn translate_markdown(&self, input: &str, target_language: &str) -> Result<String> {
        let response = self
            .azure_client
            .send_openai_request::<OpenAiResponse>(
                &OpenAiRequest {
                    messages: vec![Message {
                        role: String::from("user"),
                        content: String::from("Tell me a joke"),
                        //content: input.to_string()
                    }],
                    max_tokens: 4096,
                    temperature: 0,
                    top_p: 1,
                    model: "gpt-4o".to_string(),
                },
            )
            .await?;

        let translation = &response
            .choices
            .first()
            .unwrap()
            .message
            .content;

        Ok(translation.clone())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiRequest {
    pub messages: Vec<Message>,
    pub max_tokens: i64,
    pub temperature: i64,
    pub top_p: i64,
    pub model: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Message {
    pub role: String,
    pub content: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiCompletion {
    pub content: String,
    pub role: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Choice {
    pub index: i64,
    pub message: OpenAiCompletion,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiResponse {
    pub choices: Vec<Choice>,
}