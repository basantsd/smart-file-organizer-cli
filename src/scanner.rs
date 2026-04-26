use walkdir::WalkDir;
use std::path::PathBuf;

/// Scan directory and return list of files
pub fn scan_files(path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // skip errors for now
        };

        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    files
}