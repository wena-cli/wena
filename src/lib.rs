pub mod components;
pub mod commands;
pub mod input;
pub mod output;
pub mod support;

pub use application::{Application, ApplicationOptions};
pub use components::Alert;
pub use input::Input;
pub use output::Output;

pub type Command = commands::Command<Argv, Console>;

mod application;
mod runner;

use clap::Arg;
use input::Argv;
use output::Console;

pub fn app(name: impl Into<String>) -> Application<Argv, Console> {
    Application::new({
        ApplicationOptions {
            name: name.into().as_str(),
            version: "1.0.0",
            commands: vec![],
            input: Box::new(Argv::new()),
            output: Box::new(Console::new()),
        }
    })
}

pub fn argument(name: &str) -> clap::Arg {
    Arg::new(name).takes_value(true).required(true)
}

pub fn option(name: &str) -> clap::Arg {
    Arg::new(name).takes_value(false).required(false)
}

pub fn command(name: &str) -> Command {
    Command::new(name)
}

