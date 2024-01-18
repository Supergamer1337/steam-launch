use clap::{arg, Parser};
use itertools::Itertools;

/// Simple program to have multiple custom launch options for
/// steam games.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The command to launch the game with.
    #[arg(short, long, num_args(2), value_names = ["Launch Option Name", "Launch Command"])]
    launch: Vec<String>,

    /// This is where you need to put %command% in the launch options.
    _trailing_game_command: String,
}

impl Args {
    fn refine(self) -> RefinedArgs {
        let launch = self
            .launch
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|mut chunk| LaunchOption {
                name: chunk.next().unwrap(),
                command: chunk.next().unwrap(),
            })
            .collect();

        RefinedArgs {
            launch_options: launch,
        }
    }
}

pub struct RefinedArgs {
    pub launch_options: Vec<LaunchOption>,
}

pub struct LaunchOption {
    pub name: String,
    pub command: String,
}

pub fn args() -> RefinedArgs {
    Args::parse().refine()
}
