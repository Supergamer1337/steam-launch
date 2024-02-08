use clap::{arg, Parser};

/// Simple program to have multiple custom launch options for
/// steam games.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Extra program to launch before the game.
    #[arg(short = 'e', long = "extra", num_args = 1)]
    extra_programs: Vec<String>,

    /// Backup the save files using the ludusavi `wrap` command. Requires
    /// the name of the game as known by ludusavi.
    #[arg(short = 's', long = "save", name = "LUDUSAVI_GAME_NAME")]
    ludusavi_save: Option<String>,

    /// The commands required to launch the game.
    /// If launched from Steam, this argument will be %command%.
    #[arg(last = true, required = true)]
    game_commands: Vec<String>,
}

impl Args {
    fn refine(self) -> RefinedArgs {
        RefinedArgs {
            extra_programs: self.extra_programs,
            ludusavi_save: self.ludusavi_save,
            game_command: self.game_commands[0].clone(),
            game_args: self.game_commands[1..].to_vec(),
        }
    }
}

#[derive(Debug)]
pub struct RefinedArgs {
    pub extra_programs: Vec<String>,
    pub ludusavi_save: Option<String>,
    pub game_command: String,
    pub game_args: Vec<String>,
}

pub fn args() -> RefinedArgs {
    Args::parse().refine()
}
