use anyhow::Result;

pub trait MarkdownTranslator {
    fn translate_markdown(&self, input: &str, target_language: &str) -> Result<String>;
}

pub struct AzureTranslator;

impl MarkdownTranslator for AzureTranslator {
    fn translate_markdown(&self, input: &str, target_language: &str) -> Result<String> {
        let _ = input;

        Ok(format!("Translating '{input}' to '{target_language}'"))
    }
}
