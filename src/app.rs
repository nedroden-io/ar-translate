use std::path::{Path, PathBuf};

use anyhow::Result;
use crate::app_settings::AppConfig;
use crate::cli::RunConfig;
use crate::translator::MarkdownTranslator;

pub fn run(app_config: AppConfig, config: RunConfig, translator: &impl MarkdownTranslator) -> Result<()> {
    let _ = (config, translator);
    todo!("Implement app workflow: discover markdown files, translate, and write output")
}

fn resolve_output_root(config: &RunConfig) -> PathBuf {
    let _ = config;
    todo!("Implement output root resolution")
}

fn output_path_for(config: &RunConfig, file: &Path, output_root: Option<&Path>) -> Result<PathBuf> {
    let _ = (config, file, output_root);
    todo!("Implement destination path mapping")
}

