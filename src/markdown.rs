use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::WalkDir;

pub fn collect_markdown_files(target_path: &Path) -> Result<Vec<PathBuf>> {
    let paths = WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name().to_string_lossy().ends_with(".md"))
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    Ok(paths)
}

pub fn is_markdown_file(path: &Path) -> bool {
    let _ = path;
    todo!("Implement markdown extension detection")
}
