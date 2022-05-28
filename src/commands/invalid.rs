use crate::commands::Command;
use crate::input::Input;
use crate::output::Output;

pub struct InvalidCommandFactory {
    // ..
}

impl InvalidCommandFactory {
    pub fn new<TInput: Input, TOutput: Output>() -> Command<TInput, TOutput> {
        Command::new("invalid")
            .description("Displays an invalid command")
            .handler(|app| {
                app.output.error("Command not found.");
            })
    }
}
