pub use colored::*;
pub mod application;
pub mod commands;
pub mod input;
pub mod output;

mod runner;

pub fn app(name: &str, version: &str, commands: Vec<commands::Command>) -> application::Application {
    app_with({ application::Options {
        name,
        version,
        commands: Some(commands),
        input: Some(input::null::new()),
        output: Some(output::console::new()),
    }})
}

pub fn app_with(options: application::Options) -> application::Application {
    application::new(options)
}

pub fn command(name: &str, description: &str, handler: commands::Handler) -> commands::Command {
    commands::new(name, description, handler)
}
