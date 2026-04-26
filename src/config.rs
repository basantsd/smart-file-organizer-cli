use serde::Deserialize;
use std::{fs, path::PathBuf};
use anyhow::{Result, Context};
use dirs::config_dir;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    // pub name: String,
    pub extensions: Option<Vec<String>>,
    pub extension: Option<String>,
    pub starts_with: Option<String>,
    pub destination: String,
}

pub fn load_config(path: &str) -> Result<Config> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("❌ Failed to read config file: {}", path))?;

    let config: Config = toml::from_str(&content)
        .with_context(|| "❌ Failed to parse TOML config")?;

    Ok(config)
}

/// Get default config path
pub fn get_default_config_path() -> Option<PathBuf> {
    if let Some(mut dir) = config_dir() {
        dir.push("smart-organizer");
        dir.push("config.toml");
        return Some(dir);
    }
    None
}