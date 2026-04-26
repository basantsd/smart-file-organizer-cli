use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

pub fn move_file(file: &Path, base_path: &str, destination: &str) -> Result<()> {
    let mut dest_dir = PathBuf::from(base_path);
    dest_dir.push(destination);

    fs::create_dir_all(&dest_dir)
        .with_context(|| "❌ Failed to create directory")?;

    let file_name = file.file_name()
        .context("❌ Invalid file name")?;

    let mut dest_path = dest_dir.clone();
    dest_path.push(file_name);

    if dest_path.exists() {
        println!("⚠️ Skipping (already exists): {}", dest_path.display());
        return Ok(());
    }

    fs::rename(file, &dest_path)
        .with_context(|| format!("❌ Failed to move file: {}", file.display()))?;

    println!("✅ Moved → {}", dest_path.display());

    Ok(())
}