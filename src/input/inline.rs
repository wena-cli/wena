use clap::{ArgMatches, Command, Error, Result};

use crate::input::Input;

pub struct InlineInput {
    arguments: Vec<String>,
    matches: Result<ArgMatches, Error>,
}

impl InlineInput {
    #[allow(dead_code)]
    pub fn new(
        binary_name: impl Into<String>,
        arguments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        // prepend the binary name to the arguments
        let arguments = arguments.into_iter().map(|argument| argument.into());
        let arguments_with_binary =
            std::iter::once(binary_name.into()).chain(arguments);

        InlineInput {
            arguments: arguments_with_binary.collect(),
            matches: Ok(ArgMatches::default()),
        }
    }
}

impl Input for InlineInput {
    fn find_matches(&mut self, command: Command) {
        let matches =
            command.try_get_matches_from(self.arguments.clone().into_iter());

        self.matches = matches;
    }

    fn matches(&self) -> Result<&ArgMatches, &Error> {
        self.matches.as_ref()
    }
}
