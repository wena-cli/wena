use clap::Command;
use std::env;

use clap::{ArgMatches, Error, Result};

use crate::input::Input;

pub struct Argv {
    arguments: Vec<String>,
    matches: Result<ArgMatches, Error>
}

impl Argv {
    pub(crate) fn new() -> Self {
        let args = env::args();

        Argv {
            arguments: args.collect(),
            matches: Ok(ArgMatches::default())
        }
    }
}

impl Input for Argv {
    fn find_matches(&mut self, command: Command) {
        let matches = command.try_get_matches_from(self.arguments.clone().into_iter());

        self.matches = matches;
    }

    fn matches(&self) -> Result<&ArgMatches, &Error>
    {
        self.matches.as_ref()
    }
}
