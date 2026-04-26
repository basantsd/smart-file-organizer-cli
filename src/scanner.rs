use walkdir::WalkDir;
use std::path::PathBuf;
use anyhow::Result;

pub fn scan_files(path: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    Ok(files)
}