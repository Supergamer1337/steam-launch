#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

mod args;
mod config;
mod extra_programs;
mod saves;

use anyhow::Result;
use args::{args, RefinedArgs};
use config::Config;
use std::process::{self, Command};

fn main() {
    let Ok(config) = Config::parse() else {
        eprintln!("Failed to parse the config.");
        process::exit(1);
    };
    let args = args();

    let Ok(extra_programs) = extra_programs::start(&args.extra_programs) else {
        eprintln!("Not all programs launched successfully. Exiting...");
        process::exit(1);
    };

    if let Err(err) = launch_game(&args) {
        eprintln!("Error occurred while launching/running game: {}", err);
    }

    if args.ludusavi_save.is_some() {
        if let Err(err) = saves::backup(&config, &args) {
            eprintln!("Failed to backup saves: {}", err);
        }
    }

    extra_programs::kill(extra_programs);
}

fn launch_game(args: &RefinedArgs) -> Result<()> {
    Command::new(&args.game_command)
        .args(&args.game_args)
        .spawn()?
        .wait()?;

    Ok(())
}
