use crate::commands::Command;
use crate::components::Alert;
use crate::input::Input;
use crate::output::Output;

pub struct InvalidCommandFactory {
    // ..
}

impl InvalidCommandFactory {
    pub fn make<TInput: Input, TOutput: Output>() -> Command<TInput, TOutput> {
        Command::new("invalid")
            .io::<TInput, TOutput>()
            .description("Displays an invalid command")
            .handler(|app| {
                match app.input.matches() {

                    | Ok(_) => {
                        // ..
                    }

                    | Err(error) => {
                        app.output.writeln(Alert::error(
                            error.kind().to_string().as_str(),
                        ));
                    }
                }

                Ok(1)
            })
    }
}
