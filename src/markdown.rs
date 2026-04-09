use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

pub fn collect_markdown_files(target_path: &Path) -> Result<Vec<PathBuf>> {
    let paths = WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_markdown_file)
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    Ok(paths)
}

pub fn is_markdown_file(path: &DirEntry) -> bool {
    path.file_type().is_file() && path.file_name().to_string_lossy().ends_with(".md")
}

pub fn save_translated_file(path: &Path, body: &str, target_language: &str) -> Result<()> {
    let mut new_path = path.to_path_buf();
    let file_stem = new_path.file_stem().unwrap().to_string_lossy();
    let extension = new_path.extension().unwrap_or_default().to_string_lossy();

    new_path.set_file_name(format!("{}-{}.{}", file_stem, target_language, extension));

    std::fs::write(new_path, body)?;

    Ok(())
}