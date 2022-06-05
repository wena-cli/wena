use clap::{ArgMatches, Error, Result, Command};

use crate::input::Input;

pub struct Null {
    matches: Result<ArgMatches, Error>
}

impl Null {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Null {
            matches: Ok(ArgMatches::default())
        }
    }
}

impl Input for Null {
    fn find_matches(&mut self, command: Command) {
        let matches = command.try_get_matches_from(Vec::<String>::new().iter());

        self.matches = matches;
    }

    fn matches(&self) -> Result<&ArgMatches, &Error>
    {
        self.matches.as_ref()
    }
}
