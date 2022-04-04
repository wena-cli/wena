use crate::commands::Command;
use crate::output::Output;

pub fn new<TOutput: Output>() -> Command<TOutput> {
    crate::commands::new::<TOutput>("invalid", "Displays an invalid command", |command| {
        command.output.writeln("Command not found.");
    })
}
