mod cli;
mod scanner;
mod config;
mod rules;
mod mover;
mod progress;

use clap::Parser;
use anyhow::Result;

use cli::Args;
use scanner::scan_files;
use config::{load_config, get_default_config_path};
use rules::apply_rules;
use mover::move_file;
use progress::create_progress;

use std::path::PathBuf;

fn main() -> Result<()> {
    let args = Args::parse();

    println!("📂 Path: {}", args.path);

    let config_path: PathBuf = match args.config {
        Some(path) => PathBuf::from(path),
        None => get_default_config_path()
            .expect("❌ Could not determine config directory"),
    };

    println!("⚙️ Using config: {}\n", config_path.display());

    let config = load_config(config_path.to_str().unwrap())?;
    let files = scan_files(&args.path)?;

    println!("📊 Found {} files\n", files.len());

    if args.dry_run {
        println!("🧪 DRY RUN MODE\n");
    }

    let pb = create_progress(files.len() as u64);

    for file in &files {
        let destination = apply_rules(file, &config);

        if args.dry_run {
            println!("[DRY RUN] {} → {}", file.display(), destination);
        } else {
            move_file(file, &args.path, &destination)?;
        }

        pb.inc(1);
    }

    pb.finish_with_message("✅ Done processing files");

    Ok(())
}