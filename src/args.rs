use clap::{arg, Parser};
use itertools::Itertools;

/// Simple program to have multiple custom launch options for
/// steam games.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Extra program to launch before the game.
    #[arg(short = 'e', long = "extra")]
    extra_programs: Vec<String>,
}

impl Args {
    fn refine(self, game_args: Vec<String>) -> RefinedArgs {
        RefinedArgs {
            extra_programs: self.extra_programs,
            game_command: game_args[0].clone(),
            game_args: game_args[1..].to_vec(),
        }
    }
}

pub struct RefinedArgs {
    pub extra_programs: Vec<String>,
    pub game_command: String,
    pub game_args: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct LaunchOption {
    pub name: String,
    pub command: String,
}

pub fn args() -> RefinedArgs {
    let args = std::env::args().collect::<Vec<_>>();
    let Some((args_to_parse, game_args)) = args.split_inclusive(|arg| arg == "--").collect_tuple()
    else {
        eprintln!("No game command specified.");
        std::process::exit(1);
    };

    Args::parse_from(args_to_parse).refine(game_args.to_vec())
}
