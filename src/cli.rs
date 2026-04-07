use std::path::PathBuf;
use clap::Parser;

use anyhow::Result;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct RunConfig {
    #[arg(short, long, required = true)]
    pub target_path: PathBuf,

    #[arg(short, long, required = true, value_delimiter = ',')]
    pub languages: Vec<String>,
}

pub fn parse_args() -> Result<RunConfig> {
    Ok(RunConfig::parse())
}

