mod args;

use args::args;
use std::process::Command;

fn main() {
    let args = args();

    for program in args.extra_programs {
        Command::new(&program)
            .spawn()
            .expect("Failed to launch extra program.");
    }

    Command::new(args.game_command)
        .args(args.game_args)
        .spawn()
        .expect("Failed to launch game.");
}
