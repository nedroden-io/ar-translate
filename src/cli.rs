use clap::Parser;
use std::path::PathBuf;

use anyhow::Result;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct RunConfig {
    #[arg(short, long, required = true)]
    pub target_path: PathBuf,

    #[arg(short, long, required = true, value_delimiter = ',')]
    pub languages: Vec<String>,

    #[arg(short, long, required = false)]
    pub max_depth: Option<usize>,
}

pub fn parse_args() -> Result<RunConfig> {
    Ok(RunConfig::parse())
}
