use std::process::Command;

use clap::Parser;

/// Simple program to have multiple custom launch options for
/// steam games.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The command to launch the game with.
    #[arg(short, long)]
    launch: String,
}

fn main() {
    let args = Args::parse();

    Command::new(args.launch)
        .spawn()
        .expect("Failed to launch game.");
}
