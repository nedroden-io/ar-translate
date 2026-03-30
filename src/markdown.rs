use std::path::{Path, PathBuf};

use anyhow::Result;

pub fn collect_markdown_files(target_path: &Path) -> Result<Vec<PathBuf>> {
    let _ = target_path;
    todo!("Implement recursive markdown file discovery")
}

pub fn is_markdown_file(path: &Path) -> bool {
    let _ = path;
    todo!("Implement markdown extension detection")
}
