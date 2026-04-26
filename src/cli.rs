use clap::Parser;

/// Smart File Organizer CLI
#[derive(Parser, Debug)]
#[command(
    name = "smart-organizer",
    version,
    about = "Organize files automatically by rules"
)]
pub struct Args {
    /// Path to folder to organize
    pub path: String,

    /// Path to config file (optional)
    #[arg(short, long)]
    pub config: Option<String>,

    /// Dry run (don't move files)
    #[arg(long)]
    pub dry_run: bool,
}