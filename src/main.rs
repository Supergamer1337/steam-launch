#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

mod args;
mod config;

use args::args;
use config::Config;
use std::process::Command;

fn main() {
    let config = Config::get();
    let args = args();

    let mut extra_running = vec![];
    for program in args.extra_programs {
        extra_running.push(
            Command::new(&program)
                .spawn()
                .expect("Failed to launch extra program."),
        );
    }

    let _ = Command::new(args.game_command)
        .args(args.game_args)
        .spawn()
        .expect("Failed to launch game.")
        .wait();

    for mut process in extra_running {
        let _ = process.kill();
    }
}
