use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};

#[derive(Deserialize, Serialize)]
pub struct Config {
    ludusavi: Option<LudusaviConfig>,
}

#[derive(Deserialize, Serialize)]
struct LudusaviConfig {
    path: String,
}

impl Config {
    pub fn parse() -> Result<Config> {
        let exe_path = std::env::current_exe()?;
        let Some(config_path) = exe_path.parent().map(|p| p.join("config.toml")) else {
            return Err(anyhow!("Failed to get executable directory."));
        };

        let config = read_or_create_default_config(&config_path)?;

        Ok(config)
    }

    pub fn ludusavi_enabled(&self) -> bool {
        self.ludusavi.is_some()
    }

    pub fn ludusavi_path(&self) -> Option<&str> {
        match &self.ludusavi {
            Some(l) => Some(&l.path),
            None => None,
        }
    }
}

fn read_or_create_default_config(config_path: &PathBuf) -> Result<Config> {
    if !config_path.exists() {
        let default_config = Config { ludusavi: None };

        Ok(default_config)
    } else {
        let contents = fs::read_to_string(config_path)?;

        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }
}
