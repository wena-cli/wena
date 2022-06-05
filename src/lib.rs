pub type Argument = input::Argument;
pub type Command<TInput = ArgvInput, TOutput = ConsoleOutput> =
    commands::Command<TInput, TOutput>;

pub use application::Application;
pub use components::Alert;
pub use input::{InlineInput, Input};
pub use output::{BufferOutput, Output};

mod application;
mod commands;
mod components;
mod input;
mod output;
mod runner;
mod support;

use input::ArgvInput;
use output::ConsoleOutput;
