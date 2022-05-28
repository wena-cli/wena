use colored::*;

use crate::commands::Command;
use crate::input::Input;
use crate::output::Output;

pub struct ListCommandFactory {
    // ..
}

impl ListCommandFactory {
    pub fn make<TInput: Input, TOutput: Output>() -> Command<TInput, TOutput> {
        Command::new("list")
            .description("Displays the application commands")
            .handler(|app| {
                let name = app.name.clone().bold().white();
                let version = app.version.clone().green().bold();

                app.output.writeln("".to_string());
                app.output.writeln(format!("  {} : {}", name, version));

                let executable = std::env::current_exe().unwrap();
                let binary = executable.file_name().unwrap().to_str().unwrap();
                let usage = "USAGE:".bold().yellow();

                app.output.writeln("".to_string());
                app.output.writeln(format!(
                    "  {} {} <command> [options] [flags]",
                    usage, binary
                ));

                for subcommand in &app.commands {
                    let name = subcommand.name.clone().bold().white();
                    // let description = subcommand.description.clone().white();

                    app.output.writeln(format!("         wena {}", name));
                }

                app.output.writeln(String::from(""));
            })
    }
}
