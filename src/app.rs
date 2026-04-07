use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::app_settings::AppConfig;
use crate::cli::RunConfig;
use crate::translator::MarkdownTranslator;
use crate::markdown;

pub fn run(app_config: AppConfig, config: RunConfig, translator: &impl MarkdownTranslator) -> Result<()> {
    let markdown_files = markdown::collect_markdown_files(&config.target_path)?;

    for file in markdown_files {
        let content = std::fs::read_to_string(&file)?;
        let translation = translator.translate_markdown(&content, "es")?;
        println!("{}", translation);
    }

    Ok(())
}

fn resolve_output_root(config: &RunConfig) -> PathBuf {
    let _ = config;
    todo!("Implement output root resolution")
}

fn output_path_for(config: &RunConfig, file: &Path, output_root: Option<&Path>) -> Result<PathBuf> {
    let _ = (config, file, output_root);
    todo!("Implement destination path mapping")
}

