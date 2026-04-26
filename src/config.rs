use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: String,
    pub extensions: Option<Vec<String>>,
    pub extension: Option<String>,
    pub starts_with: Option<String>,
    pub destination: String,
}

/// Load config from TOML file
pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path)
        .expect("❌ Failed to read config file");

    toml::from_str(&content)
        .expect("❌ Failed to parse TOML")
}