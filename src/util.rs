use anyhow::{anyhow, Result};
use std::{
    path::PathBuf,
    process::{Child, Command},
};
use tracing::debug;

pub fn get_path_relative_to_exe(path: &str) -> Result<PathBuf> {
    std::env::current_exe()?
        .parent()
        .map(|p| p.join(path))
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory of executable"))
}

pub struct AdminRunner {
    command: Command,
}

impl AdminRunner {
    pub fn new(cmd: &str) -> Self {
        let mut command = Command::new("powershell");

        debug!("Before changing command: {}", cmd);
        let mut changed_command = cmd.to_string();
        if cmd.contains(' ') {
            changed_command = "\"".to_string() + cmd + "\"";
        }
        debug!("After changing command: {}", changed_command);

        command
            .arg("-WindowStyle")
            .arg("Hidden")
            .arg("-Command")
            .arg("Start-Process")
            .arg("-FilePath")
            .arg(changed_command);

        AdminRunner { command }
    }

    pub fn kill(program: &mut Child) -> Result<()> {
        let pid = "/PID ".to_string() + &program.id().to_string();
        AdminRunner::new("taskkill")
            .with_args(&["/F", "/T", &pid])
            .run()
            .map_err(|e| anyhow!(e))?
            .wait()
            .map_err(|e| anyhow!(e))?;

        Ok(())
    }

    pub fn with_args(mut self, args: &[&str]) -> Self {
        let arguments = "\"".to_string() + &args.join("\",\"") + "\"";
        self.command.arg("-ArgumentList").arg(arguments);
        self
    }

    pub fn run(mut self) -> Result<Child> {
        self.command
            .arg("-Verb")
            .arg("RunAs")
            .arg("-Wait")
            .spawn()
            .map_err(|e| anyhow!(e))
    }
}
