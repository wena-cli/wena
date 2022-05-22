use crate::commands::*;
use crate::input::Input;
use crate::output::Output;

pub fn new<TInput: Input, TOutput: Output>() -> Command<TInput, TOutput> {
    crate::commands::new::<TInput, TOutput>("invalid", "Displays an invalid command", |app| {
        app.output.error("Command not found.");
    })
}
