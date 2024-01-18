mod args;

use std::process::Command;

use args::{args, LaunchOption};
use handlebars::Handlebars;
use serde_json::json;

fn main() {
    let args = args();

    let picker = create_launch_picker(args.launch_options.iter().map(|x| &x.name).collect());

    let mut choice = None;
    web_view::builder()
        .title("Launch Options")
        .content(web_view::Content::Html(picker))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            if args
                .launch_options
                .iter()
                .filter(|option| option.name == arg)
                .count()
                > 0
            {
                choice = Some(arg.to_string());
                return Ok(webview.exit());
            }

            Ok(())
        })
        .run()
        .unwrap();

    let choice = choice.unwrap();
    let option = args
        .launch_options
        .iter()
        .find(|x| x.name == choice)
        .unwrap()
        .clone();

    Command::new(option.command)
        .spawn()
        .expect("Failed to launch game.");
}

fn create_launch_picker(names: Vec<&String>) -> String {
    let mut h = Handlebars::new();
    let picker = include_str!("../markup/picker.html");
    h.register_template_string("picker", picker).unwrap();
    h.render("picker", &json!({"launchOptions": names}))
        .unwrap()
}
