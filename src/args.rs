use clap::{arg, Parser};

/// Simple program to have multiple custom launch options for
/// steam games.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Extra program to launch before the game.
    #[arg(short = 'e', long = "extra")]
    extra_programs: Vec<String>,

    /// The commands required to launch the game.
    /// If launched from Steam, this argument will be %command%.
    #[arg(last = true)]
    game_commands: Vec<String>,
}

impl Args {
    fn refine(self) -> RefinedArgs {
        RefinedArgs {
            extra_programs: self.extra_programs,
            game_command: self.game_commands[0].clone(),
            game_args: self.game_commands[1..].to_vec(),
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
    Args::parse().refine()
}
