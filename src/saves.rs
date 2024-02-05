use std::process::Command;

use crate::args::RefinedArgs;
use crate::config::Config;
use anyhow::{anyhow, Result};

pub fn backup(config: &Config, args: &RefinedArgs) -> Result<()> {
    if !config.ludusavi_enabled() {
        return Err(anyhow!("Ludusavi is not enabled in the config."));
    }

    let ludusavi_path = config
        .ludusavi_path()
        .ok_or(anyhow!("Ludusavi path not found."))?;

    let game_name = args
        .ludusavi_save
        .as_ref()
        .ok_or(anyhow!("Ludusavi game name not found."))?;

    Command::new(ludusavi_path)
        .arg("backup")
        .arg("--force")
        .arg(game_name)
        .spawn()?
        .wait()?;

    Ok(())
}
