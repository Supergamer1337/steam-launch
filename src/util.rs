use anyhow::Result;
use std::path::PathBuf;

pub fn get_path_relative_to_exe(path: &str) -> Result<PathBuf> {
    std::env::current_exe()?
        .parent()
        .map(|p| p.join(path))
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory of executable"))
}
