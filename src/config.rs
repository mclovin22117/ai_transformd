use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub provider_url: Option<String>,
    pub api_keys: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config { provider_url: None, api_keys: vec![] }
    }
}

impl Config {
    pub fn load_from(path: PathBuf) -> Self {
        if path.exists() {
            let s = std::fs::read_to_string(path).unwrap_or_default();
            toml::from_str(&s).unwrap_or_default()
        } else {
            Config::default()
        }
    }
}
