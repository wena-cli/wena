use crate::input::Input;
pub use colored::*;
pub mod application;
pub mod commands;
pub mod input;
pub mod output;
pub use input::null::*;
pub use output::console::*;
pub use output::Output;

use crate::input::null;
use crate::output::console;

mod runner;

pub fn app(name: &str, version: &str, commands: Vec<commands::Command<null::Null, console::Console>>) -> application::Application<null::Null, console::Console> {
    from::<Null, Console>({ application::Options {
        name,
        version,
        commands,
        input: null::new(),
        output: console::new(),
    }})
}

pub fn from<TInput : Input, TOutput : Output>(options: application::Options<TInput, TOutput>) -> application::Application<TInput, TOutput>  {
    application::new(options)
}

pub fn command<TInput : Input, TOutput : Output>(name: &str, description: &str, handler: commands::Handler<TInput, TOutput>) -> commands::Command<TInput, TOutput> {
    commands::new::<TInput, TOutput>(name, description, handler)
}
