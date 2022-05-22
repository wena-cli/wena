use clap::Arg;
use crate::input::Input;
use crate::output::Output;
use crate::application::Application;
use clap::Command;

pub fn run<TInput : Input, TOutput : Output>(
    application: &mut Application<TInput, TOutput>,
) -> &Application<TInput, TOutput> {
    let mut command = Command::new(&application.name);

    for subcommand in &application.commands {

        let mut args = subcommand
            .tokens
            .iter();

        args.next();

        command = command.subcommand(
            Command::new(subcommand.name.to_string()).args(
                args.clone().map(|argument| Arg::new(argument.as_str()).takes_value(true))
            )
        );
    }

    match command.try_get_matches_from(application.input.to_iter()) {
        Ok(_matches) => {
            let input = application.input.with_arguments_matches(_matches.clone());

            application.input = input;

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
