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
            .argument("version", |argument| argument.takes_value(false).short('v'))
            .handler(|app| {
                let executable = std::env::current_exe().unwrap();
                let binary = executable.file_name().unwrap().to_str().unwrap();
                let usage = "USAGE:".bold().yellow();

                app.output.new_line();
                app.output.writeln(format!(
                    "  {} {} <command> [arguments] [--flags]",
                    usage, binary
                ));

                for subcommand in &app.commands {
                    let name = subcommand.name.clone().bold().white();
                    // let description = subcommand.description.clone().white();

                    app.output.write(format!("         {} {}", binary, name));
                    subcommand.arguments.iter().for_each(|argument| {
                        let name = argument.get_id().clone();
                        // let description = argument.long().clone().white();

                        app.output.write(format!(" [{}]", name));
                    });
                }

                app.output.new_line();
            })
    }
}
