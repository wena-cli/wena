use std::env;

use clap::ArgMatches;

use crate::input::Input;

pub fn new() -> Box<Argv> {
    let args = env::args();

    Box::new(Argv {
        arguments: args.collect(),
        arguments_matches: None,
    })
}

pub struct Argv {
    arguments: Vec<String>,
    arguments_matches: Option<ArgMatches>,
}

impl Input for Argv {
    fn argument(&self, name: &str) -> &str {
        if let Some(arguments_matches) = &self.arguments_matches {
            if let Some(value) = arguments_matches.value_of(name) {
                return value;
            }
        }

        panic!("Argument not found: {}.", name);
    }

    fn with_arguments_matches(&self, matches: ArgMatches) -> Box<Argv> {
        Box::new(Argv {
            arguments: self.arguments.clone(),
            arguments_matches: Some(matches),
        })
    }

    fn to_iter(&self) -> Box<dyn Iterator<Item = String>> {
        Box::new(self.arguments.clone().into_iter())
    }
}
