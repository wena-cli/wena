use crate::output::Output;
use crate::input::Input;
use crate::commands::Command;

pub fn new<TInput : Input, TOutput : Output>() -> Command<TInput, TOutput> {
    crate::commands::new::<TInput, TOutput>("invalid", "Displays an invalid command", |app| {
        app.output.writeln("Command not found.");
    })
}
