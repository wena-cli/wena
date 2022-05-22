pub use colored::*;
pub mod application;
pub mod commands;
pub mod input;
pub mod output;
pub use input::null::*;
pub use input::inline::*;
pub use output::console::*;
pub use output::Output;
pub use input::argv::Argv;
pub use input::Input;

use crate::input::argv;
use crate::output::console;

mod runner;

pub fn app(name: &str, version: &str, commands: Vec<commands::Command<Argv, console::Console>>) -> application::Application<Argv, console::Console> {
    from::<Argv, Console>({ application::Options {
        name,
        version,
        commands,
        input: argv::new(),
        output: console::new(),
    }})
}

pub fn from<TInput : Input, TOutput : Output>(options: application::Options<TInput, TOutput>) -> application::Application<TInput, TOutput>  {
    application::new(options)
}

pub fn command<TInput : Input, TOutput : Output>(name: &str, description: &str, handler: commands::Handler<TInput, TOutput>) -> commands::Command<TInput, TOutput> {
    commands::new::<TInput, TOutput>(name, description, handler)
}