use std::path::PathBuf;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub target_path: PathBuf,
    pub target_language: String,
    pub in_place: bool,
    pub output_dir: Option<PathBuf>,
}

pub fn parse_args<I, S>(args: I) -> Result<AppConfig>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let _ = args;
    todo!("Implement CLI argument parsing")
}

