use colored::Color::TrueColor;
use colored::*;

use crate::commands::Command;
use crate::input::Input;
use crate::output::Output;
use crate::support::Terminal;

pub struct ListCommandFactory {
    // ..
}

impl ListCommandFactory {
    pub fn make<TInput: Input, TOutput: Output>() -> Command<TInput, TOutput> {
        Command::new("list")
            .io::<TInput, TOutput>()
            .description("Displays the application commands")
            .handler(|app| {
                let executable = std::env::current_exe().unwrap();
                let binary = executable.file_name().unwrap().to_str().unwrap();
                let usage = "USAGE:".bold().yellow();

                app.output.new_line();
                app.output.writeln(format!(
                    "  {} {} <command> [arguments] [--flags]",
                    usage, binary
                ));

                app.output.new_line();

                let terminal = Terminal::default();

                for subcommand in &app.commands {
                    let name = subcommand.name.clone().bold().white();
                    let description = subcommand.description.clone();

                    let mut line = format!("  {}", name);

                    subcommand.definition.iter().for_each(|argument| {
                        let name = argument.get_id();
                        line.push_str(&format!(" [{}]", name).color(
                            TrueColor {
                                r: 100,
                                g: 100,
                                b: 100,
                            },
                        ));
                    });

                    line.push(' ');

                    line.push_str(
                        format!(
                            "{}",
                            ".".repeat(
                                terminal.width() - line.len() + 8
                                    - description.len()
                            )
                            .color(TrueColor {
                                r: 100,
                                g: 100,
                                b: 100
                            })
                        )
                        .as_str(),
                    );

                    line.push(' ');
                    line.push_str(description.as_str());

                    app.output.writeln(line);
                }

                if (app.commands.len() as i32) > 0 {
                    app.output.new_line();
                }

                Ok(0)
            })
    }
}
