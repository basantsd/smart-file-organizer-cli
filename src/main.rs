use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
// #[command(author, version, about)]
struct Args {
    /// Folder path to organize
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("Scanning folder: {}", args.path);

    for entry in WalkDir::new(&args.path) {
        let entry = entry.unwrap();

        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    }
}