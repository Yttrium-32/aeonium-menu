use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use anyhow::Ok;
use tracing::{warn, info};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config { timeout: 100 }
    }
}

fn create_default_config(file_path: &Path) -> anyhow::Result<()> {
    let mut config_file = File::create(file_path)?;
    let config_vals = toml::to_string(&Config::default())?;

    config_file.write_all(&config_vals.into_bytes())?;

    Ok(())
}

impl Config {
    pub fn parse(proj_dirs: &ProjectDirs) -> anyhow::Result<Self> {
        let config_dir = proj_dirs.config_dir();
        let config_file_path = config_dir.join("config.toml");

        if !config_dir.exists() {
            info!("Config dir doesn't exit, creating...");
            fs::create_dir_all(config_dir)?;
        }

        if !config_file_path.is_file() {
            warn!("Config file not found, creating default config...");
            create_default_config(&config_file_path)?;
        }

        let config_str = fs::read_to_string(config_file_path)?;
        let config_vals = toml::from_str(&config_str)?;

        Ok(config_vals)
    }
}
