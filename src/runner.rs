use crate::application::Application;
use crate::input::Input;
use crate::Output;
use clap::Command;

pub fn run<TInput: Input, TOutput: Output>(
    application: &mut Application<TInput, TOutput>,
) -> &Application<TInput, TOutput> {
    let mut command = Command::new(&application.name);

    for subcommand in &application.commands {
        command = command.subcommand(
            Command::new(subcommand.name.to_string()).args(subcommand.arguments.clone()),
        );
    }

    match command.try_get_matches_from(application.input.to_iter()) {
        Ok(matches) => {
            let matched_command = matches.subcommand();

            if matched_command.is_some() {
                let (name, mached_command_matches) = matched_command.unwrap();

                application.input = application
                    .input
                    .with_arguments_matches(mached_command_matches.clone());

                let subcommand = application
                    .commands
                    .iter()
                    .find(|subcommand| subcommand.name == name)
                    .unwrap();

                for argument in &subcommand.arguments {
                    if !mached_command_matches.is_present(argument.get_id()) {
                        application
                            .output
                            .error(&format!("Missing argument: {}.", argument.get_id()));

                        return application;
                    }
                }

                (subcommand.handler)(application);
            } else {
                (crate::commands::list::new().handler)(application);
            }
        }
        Err(_) => {
            (crate::commands::invalid::new().handler)(application);
        }
    }

    application
}
