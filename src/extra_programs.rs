use crate::util::AdminRunner;
use anyhow::{anyhow, Result};
use std::process::{Child, Command};

trait Ignore: Sized {
    fn ignore(self) {}
}

impl<T, E> Ignore for Result<T, E> {}

type Programs = Vec<Child>;

pub fn start(programs: &[String]) -> Result<Programs> {
    programs
        .iter()
        .map(|p| Command::new(p).spawn())
        .map(|r| r.map_err(|e| anyhow!(e)))
        .collect()
}

pub fn start_admin(programs: &[String]) -> Result<Programs> {
    programs.iter().map(|p| AdminRunner::new(p).run()).collect()
}

pub fn kill(programs: Programs) {
    programs.into_iter().for_each(|mut p| p.kill().ignore());
}

pub fn kill_admin(programs: Programs) {
    programs
        .into_iter()
        .for_each(|mut p| AdminRunner::kill(&mut p).ignore());
}
