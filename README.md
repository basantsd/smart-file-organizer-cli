# Smart File Organizer CLI (Rust)

A fast, flexible, and safe command-line tool built in Rust to automatically organize messy folders into structured directories using customizable rules.

---

## Why This Project?

Most people’s **Downloads folder is chaos** — images, PDFs, videos, code files all mixed together.

This tool solves that problem by:

* Scanning your folders automatically
* Applying smart, customizable rules
* Organizing files into clean, structured directories
* Doing it safely with preview (dry-run) support

---

## Features

* ✅ Recursive file scanning
* ✅ Rule-based file organization (via TOML config)
* ✅ Dry-run mode (preview before moving files)
* ✅ Safe file handling (no overwrites)
* ✅ Progress bar for real-time feedback
* ✅ Cross-platform support (Linux, macOS, Windows)
* ✅ Clean error handling (no crashes)

---

## How It Works

1. You provide a folder (e.g., Downloads)
2. The tool scans all files recursively
3. It applies rules defined in a config file
4. Files are moved into categorized folders

---

## Example Usage

```bash
# Preview (safe mode)
smart-organizer ~/Downloads --dry-run

# With custom config
smart-organizer ~/Downloads --config config.toml

# Actual execution
smart-organizer ~/Downloads
```

---

## Configuration (TOML)

Create a `config.toml` file:

```toml
[[rules]]
name = "Images"
extensions = ["jpg", "png", "jpeg"]
destination = "Images"

[[rules]]
name = "Documents"
extensions = ["pdf", "docx", "txt"]
destination = "Documents"

[[rules]]
name = "Reports"
extension = "csv"
starts_with = "report_"
destination = "Data"
```

---

## 🧪 Dry Run Example

```bash
smart-organizer ~/Downloads --dry-run
```

Output:

```
[DRY RUN] photo.jpg → Images/
[DRY RUN] report_2024.csv → Data/
```

---

## 📊 Real Execution Example

```
✅ Moved → Downloads/Images/photo.jpg
✅ Moved → Downloads/Documents/file.pdf
⚠️ Skipped (already exists)
```

---

## Project Structure

```
src/
 ├── main.rs        # Entry point
 ├── cli.rs         # CLI argument parsing
 ├── scanner.rs     # File scanning logic
 ├── config.rs      # Config loader (TOML)
 ├── rules.rs       # Rule engine (core logic)
 ├── mover.rs       # File moving logic
 ├── progress.rs    # Progress bar UI
```

---

## Tech Stack

* Rust 🦀
* clap — CLI parsing
* walkdir — directory traversal
* serde + toml — config parsing
* anyhow — error handling
* dirs — system config paths
* indicatif — progress bar

---

## 🛡️ Safety Features

* ✔ Dry-run mode (preview before execution)
* ✔ No file overwrite (skips duplicates)
* ✔ Graceful error handling
* ✔ Controlled file movement

---

## Installation

```bash
cargo build --release
```

Run:

```bash
./target/release/smart-file-organizer ~/Downloads
```

---

## 👨‍💻 Author

Built by Basant Singh using Rust to solve a real-world problem.

---

## ⭐ Support

If you like this project, consider giving it a ⭐ on GitHub!
