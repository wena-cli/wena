use crate::input::Input;
use crate::output::Output;
use crate::application::Application;
use clap::Command;

pub fn run<TInput : Input, TOutput : Output>(
    application: &mut Application<TInput, TOutput>,
) -> &Application<TInput, TOutput> {
    let mut command = Command::new(&application.name);

    for subcommand in &application.commands {
        command = command.subcommand(Command::new(subcommand.name.to_string()));
    }

    match command.try_get_matches_from(application.input.to_iter()) {
        Ok(_matches) => {
            let subcommand = _matches.subcommand();

            if subcommand.is_some() {
                let name = _matches.subcommand().unwrap().0;

                (&application
                    .commands
                    .iter()
                    .find(|command| command.name == name)
                    .unwrap()
                    .handler)(application);
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
