#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

mod args;
mod config;
mod extra_programs;
mod log;
mod saves;
mod util;

use anyhow::Result;
use args::{args, RefinedArgs};
use config::Config;
use std::process::{self, Command};
use tracing::{debug, error, info};

fn main() {
    if let Err(_) = log::enable() {
        eprintln!("Failed to enable logging. Exiting...");
        process::exit(1);
    }

    info!("Logging enabled. Starting...");
    info!("Parsing config file...");
    let config = match Config::parse() {
        Ok(c) => c,
        Err(err) => {
            error!("Failed to parse config file");
            debug!("Error: {}", err);
            process::exit(1);
        }
    };

    info!("Parsing command line arguments...");
    let args = args();
    debug!("Arguments: {:?}", args);

    let mut extra_programs = vec![];
    if !args.extra_programs.is_empty() {
        info!("Extra programs given. Starting them...");
        match extra_programs::start(&args.extra_programs) {
            Ok(p) => extra_programs = p,
            Err(err) => {
                debug!("Error while starting programs: {}", err);
                error!("Some program failed to start. Exiting...");
                process::exit(1);
            }
        };
    } else {
        info!("No extra programs to start. Continuing...");
    }

    info!("Launching game...");
    if let Err(err) = launch_game(&args) {
        error!("Error occurred while launching/running game");
        info!("Error: {}", err);
    }

    info!("Game exited. Starting post-game tasks...");
    if args.ludusavi_save.is_some() {
        info!("Backing up saves...");
        if let Err(err) = saves::backup(&config, &args) {
            error!("Failed to backup saves");
            info!("Error: {}", err);
        }
    }

    if !args.extra_programs.is_empty() {
        info!("Killing extra programs...");
        extra_programs::kill(extra_programs);
    }

    info!("Exiting...");
}

fn launch_game(args: &RefinedArgs) -> Result<()> {
    Command::new(&args.game_command)
        .args(&args.game_args)
        .spawn()?
        .wait()?;

    Ok(())
}
