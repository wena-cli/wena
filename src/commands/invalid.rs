use crate::commands::Command;

pub fn new() -> Command {
    crate::commands::new("invalid", "Displays an invalid command", |command| {
        command.output.writeln("Command not found.");
    })
}
