use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::{WalkDir, DirEntry};

pub fn collect_markdown_files(target_path: &Path) -> Result<Vec<PathBuf>> {
    let paths = WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_markdown_file(e))
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    Ok(paths)
}

pub fn is_markdown_file(path: &DirEntry) -> bool {
    path.file_type().is_file() && path.file_name().to_string_lossy().ends_with(".md")
}
