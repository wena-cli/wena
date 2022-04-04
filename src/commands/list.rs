use crate::commands::Command;
use crate::output::Output;

pub fn new<TOutput: Output>() -> Command<TOutput> {
    crate::commands::new::<TOutput>("list", "Displays the application commands", |command| {
        let name = command.application.name.clone();
        let version = command.application.version.clone();

        command.output.writeln("");
        command.output.writeln(&format!("  {}: {}", name, version));
        command.output.writeln("");
    })
}
