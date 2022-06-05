use clap::{ArgMatches, Error, Result, Command};

use crate::input::Input;

pub struct Inline {
    arguments: Vec<String>,
    matches: Result<ArgMatches, Error>
}

impl Inline {
    #[allow(dead_code)]
    pub fn new(binary_name: String, arguments: Vec<String>) -> Self {
        let append = vec![binary_name];

        Inline {
            arguments: [&append[..], &arguments[..]].concat(),
            matches: Ok(ArgMatches::default())
        }
    }
}

impl Input for Inline {
    fn find_matches(&mut self, command: Command) {
        let matches = command.try_get_matches_from(self.arguments.clone().into_iter());

        self.matches = matches;
    }

    fn matches(&self) -> Result<&ArgMatches, &Error>
    {
        self.matches.as_ref()
    }
}
