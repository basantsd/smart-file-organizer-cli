mod cli;
mod scanner;
mod config;
mod rules;

use clap::Parser;
use cli::Args;
use scanner::scan_files;
use config::load_config;
use rules::apply_rules;

fn main() {
    let args = Args::parse();

    println!("Path: {}", args.path);

    let config_path = args.config.unwrap_or("config.toml".to_string());
    let config = load_config(&config_path);

    let files = scan_files(&args.path);

    println!("Found {} files\n", files.len());

    // Apply rules to each file
    for file in files.iter().take(10) {
        let destination = apply_rules(file, &config);

        println!(
            "{} → {}",
            file.display(),
            destination
        );
    }
}