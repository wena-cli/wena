use clap::{ArgMatches, Command, Error, Result};

use crate::input::Input;

pub struct NullInput {
    matches: Result<ArgMatches, Error>,
}

impl NullInput {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        NullInput {
            matches: Ok(ArgMatches::default()),
        }
    }
}

impl Input for NullInput {
    fn find_matches(&mut self, command: Command) {
        let matches = command.try_get_matches_from(Vec::<String>::new().iter());

        self.matches = matches;
    }

    fn matches(&self) -> Result<&ArgMatches, &Error> {
        self.matches.as_ref()
    }
}
