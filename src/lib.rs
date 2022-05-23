pub mod application;
pub mod commands;
pub mod input;
pub mod output;
use clap::Arg;
pub use colored::*;
pub use input::argv::Argv;
pub use input::inline::*;
pub use input::null::*;
pub use input::Input;
pub use output::console::*;
pub use output::Output;

mod runner;

use crate::input::argv;
use crate::output::console;

pub fn app(name: &str) -> application::Application<Argv, console::Console> {
    from::<Argv, Console>({
        application::Options {
            name,
            version: "1.0.0",
            commands: vec![],
            input: argv::new(),
            output: console::new(),
        }
    })
}

pub fn from<TInput: Input, TOutput: Output>(
    options: application::Options<TInput, TOutput>,
) -> application::Application<TInput, TOutput> {
    application::new(options)
}

pub fn argument(name: &str) -> clap::Arg {
    Arg::new(name)
}

pub fn command<TInput: Input, TOutput: Output>(name: &str) -> commands::Command<TInput, TOutput> {
    commands::new::<TInput, TOutput>(name)
}
