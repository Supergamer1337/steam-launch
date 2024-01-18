mod args;

use args::args;

fn main() {
    let args = args();

    for (name, command) in args.launch_options.iter().map(|x| (&x.name, &x.command)) {
        println!("{}: {}", name, command);
    }

    // Command::new(args.launch)
    //     .spawn()
    //     .expect("Failed to launch game.");
}
