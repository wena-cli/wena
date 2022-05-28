pub mod input;
pub mod output;

pub use application::{Application, ApplicationOptions};
pub use commands::Command;

mod application;
mod commands;
mod runner;

use clap::Arg;
use input::{Argv, Input};
use output::{Console, Output};

pub fn app(name: &str) -> Application<Argv, Console> {
    Application::new({
        ApplicationOptions {
            name,
            version: "1.0.0",
            commands: vec![],
            input: Box::new(Argv::new()),
            output: Box::new(Console::new()),
        }
    })
}

pub fn argument(name: &str) -> clap::Arg {
    Arg::new(name)
}

pub fn command<TInput: Input, TOutput: Output>(
    name: &str,
) -> commands::Command<TInput, TOutput> {
    Command::new(name)
}
