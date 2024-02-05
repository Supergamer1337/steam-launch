#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

mod args;
mod config;

use args::{args, RefinedArgs};
use config::Config;
use std::process::Command;

fn main() {
    let config = Config::get();
    let args = args();

    let extra_programs = start_extra_programs(&args.extra_programs);

    launch_game(&args);

    if args.ludusavi_save.is_some() {
        backup_saves(&config, &args);
    }

    kill_extra_programs(extra_programs);
}

fn start_extra_programs(programs: &Vec<String>) -> Vec<std::process::Child> {
    let mut extra_running = vec![];
    for program in programs {
        extra_running.push(
            Command::new(&program)
                .spawn()
                .expect("Failed to launch extra program."),
        );
    }

    extra_running
}

fn kill_extra_programs(programs: Vec<std::process::Child>) {
    for mut program in programs {
        program.kill().expect("Failed to kill extra program.");
    }
}

fn backup_saves(config: &Config, args: &RefinedArgs) {
    if !config.ludusavi_enabled() {
        eprintln!("Ludusavi is not enabled in the config.");
        return;
    }

    let ludusavi_path = config
        .ludusavi_path()
        .expect("Ludusavi being enabled should guarantee a path.");

    let game_name = args
        .ludusavi_save
        .as_ref()
        .expect("Ludusavi save should be present.");

    let _ = Command::new(ludusavi_path)
        .arg("backup")
        .arg("--force")
        .arg(game_name)
        .spawn()
        .expect("Failed to launch ludusavi.")
        .wait();
}

fn launch_game(args: &RefinedArgs) {
    let _ = Command::new(&args.game_command)
        .args(&args.game_args)
        .spawn()
        .expect("Failed to launch game.")
        .wait();
}
