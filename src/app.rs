use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::cli::AppConfig;
use crate::translator::MarkdownTranslator;

pub fn run(config: AppConfig, translator: &impl MarkdownTranslator) -> Result<()> {
    let _ = (config, translator);
    todo!("Implement app workflow: discover markdown files, translate, and write output")
}

fn resolve_output_root(config: &AppConfig) -> PathBuf {
    let _ = config;
    todo!("Implement output root resolution")
}

fn output_path_for(config: &AppConfig, file: &Path, output_root: Option<&Path>) -> Result<PathBuf> {
    let _ = (config, file, output_root);
    todo!("Implement destination path mapping")
}

