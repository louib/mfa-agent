use std::env;
use std::fs;
use std::path;

use serde::{Deserialize, Serialize};

pub fn get_cache_dir() -> String {
    // TODO should be placed in an app-specific, xdg compliant location
    match env::home_dir() {
        Some(p) => p.display().to_string(),
        None => ".".to_string(),
    }
}

pub fn get_config_file_path() -> String {
    get_cache_dir().to_owned() + "/" + DEFAULT_CONFIG_FILE_NAME
}

pub const DEFAULT_CONFIG_FILE_NAME: &str = ".mfa-agent-config.yaml";
pub const DEFAULT_DB_FILE_NAME: &str = "mfa-agent.kdbx";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub agent_id: Option<String>,
    pub default_db_path: Option<String>,
    pub last_db_path: Option<String>,
    pub known_server_address: Option<String>,
}

pub fn write_config(config: &Config) -> Result<Config, String> {
    let cache_dir = get_cache_dir();
    let cache_dir = path::Path::new(&cache_dir);
    if !cache_dir.is_dir() {
        match fs::create_dir(cache_dir) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        };
    }

    let config_content = match serde_yaml::to_string(&config) {
        Ok(m) => m,
        Err(e) => return Err(format!("Failed to dump the config {}", e)),
    };

    let config_path = get_config_file_path();
    let config_path = path::Path::new(&config_path);
    match fs::write(config_path, config_content) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "Failed to write the config file at {}: {}",
                config_path.to_str().unwrap_or(""),
                e
            ))
        }
    };

    read()
}

pub fn read() -> Result<Config, String> {
    let config_path = get_config_file_path();
    let config_path = path::Path::new(&config_path);
    let config_content = match fs::read_to_string(config_path) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "Failed to read the config file at {}: {}.",
                config_path.to_str().unwrap_or(""),
                e,
            ))
        }
    };

    let config: Config = match serde_yaml::from_str(&config_content) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "Failed to parse the config file at {}: {}.",
                config_path.to_str().unwrap_or(""),
                e
            ))
        }
    };
    Ok(config)
}

pub fn read_or_init() -> Result<Config, String> {
    match read() {
        Ok(config) => Ok(config),
        Err(_) => match write_config(&Config::default()) {
            Ok(c) => return Ok(c),
            Err(e) => return Err(e),
        },
    }
}
