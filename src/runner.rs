use clap::Command;
use colored::Colorize;

use crate::application::Application;
use crate::components::Alert;
use crate::input::Input;
use crate::output::Output;

pub fn run<TInput: Input, TOutput: Output>(
    application: &mut Application<TInput, TOutput>,
) -> &Application<TInput, TOutput> {
    let mut command = Command::new(&application.name);

    for subcommand in &application.commands {
        command = command.subcommand(
            Command::new(subcommand.name.to_string())
                .args(subcommand.definition.clone()),
        );
    }

    application.input.find_matches(command);
    
    match application.input.matches() {
        | Ok(matches) => {
            let matched_command = matches.subcommand();

            if let Some((name, mached_command_matches)) = matched_command {
                let subcommand = application
                    .commands
                    .iter()
                    .find(|subcommand| subcommand.name == name)
                    .unwrap();

                for argument in &subcommand.definition {
                    if !mached_command_matches.is_present(argument.get_id()) && argument.is_required_set() {
                        application.output.writeln(Alert::error(&format!(
                            "Argument {} is required.",
                            argument.get_id().bold().white().italic(),
                        )));    
                    }
                }

                (subcommand.handler)(application);
            } else {
                (crate::commands::ListCommandFactory::make().handler)(
                    application,
                );
            }
        }
        | Err(_) => {
            (crate::commands::InvalidCommandFactory::make().handler)(
                application,
            );
        }
    }

    application
}
