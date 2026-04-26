use crate::config::Config;
use std::path::Path;

/// Decide destination based on rules
pub fn apply_rules(path: &Path, config: &Config) -> String {
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    for rule in &config.rules {

        // Match multiple extensions
        if let Some(exts) = &rule.extensions {
            if exts.iter().any(|e| e == &extension) {
                return rule.destination.clone();
            }
        }

        // Match single extension
        if let Some(ext) = &rule.extension {
            if ext == &extension {

                // Optional prefix check
                if let Some(prefix) = &rule.starts_with {
                    if file_name.starts_with(prefix) {
                        return rule.destination.clone();
                    }
                } else {
                    return rule.destination.clone();
                }
            }
        }
    }

    // Default fallback
    "Others".to_string()
}