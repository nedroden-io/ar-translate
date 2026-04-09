use crate::azure::AzureClient;
use anyhow::Result;

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

#[derive(serde::Serialize)]
struct TranslateRequest {
    text: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateResponse {
    detected_language: DetectedLanguage,
    translations: Vec<Translation>,
}

#[derive(serde::Deserialize)]
struct Translation {
    text: String,
    to: String,
}

#[derive(serde::Deserialize)]
struct DetectedLanguage {
    language: String,
    score: f32,
}

impl<'a> MarkdownTranslator for AzureTranslator<'a> {
    async fn translate_markdown(&self, input: &str, target_language: &str) -> Result<String> {
        let response = self
            .azure_client
            .send_openai_request::<Vec<TranslateResponse>>(
                &[TranslateRequest {
                    text: input.to_string(),
                }],
            )
            .await?;

        let default = &Translation {
            text: "???".to_string(),
            to: target_language.to_string(),
        };
        let translation = response
            .first()
            .unwrap()
            .translations
            .first()
            .unwrap_or(default);

        Ok(translation.text.clone())
    }
}
