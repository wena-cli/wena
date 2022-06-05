use std::env;

use clap::{ArgMatches, Command, Error, Result};

use crate::input::Input;

pub struct ArgvInput {
    arguments: Vec<String>,
    matches: Result<ArgMatches, Error>,
}

impl ArgvInput {
    pub(crate) fn new() -> Self {
        let args = env::args();

        ArgvInput {
            arguments: args.collect(),
            matches: Ok(ArgMatches::default()),
        }
    }
}

impl Input for ArgvInput {
    fn find_matches(&mut self, command: Command) {
        let matches =
            command.try_get_matches_from(self.arguments.clone().into_iter());

        self.matches = matches;
    }

    fn matches(&self) -> Result<&ArgMatches, &Error> {
        self.matches.as_ref()
    }
}
