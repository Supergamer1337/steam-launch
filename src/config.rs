use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub ludusavi_path: Option<String>,
}

impl Config {
    fn parse() -> Result<Config> {
        let exe_path = std::env::current_exe()?;
        let Some(config_path) = exe_path.parent().and_then(|p| Some(p.join("config.toml"))) else {
            return Err(anyhow!("Failed to get executable directory."));
        };

        let config = read_or_create_default_config(&config_path)?;

        Ok(config)
    }

    pub fn get() -> Config {
        match Config::parse() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to parse config with the following error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn read_or_create_default_config(config_path: &PathBuf) -> Result<Config> {
    if !config_path.exists() {
        let default_config = Config {
            ludusavi_path: None,
        };

        fs::write(config_path, toml::to_string(&default_config)?)?;

        Ok(default_config)
    } else {
        let contents = fs::read_to_string(config_path)?;

        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }
}
