use clap::ArgMatches;
use crate::input::{ Input };

pub fn new(arguments: Vec<String>) -> Box<Inline> {
    Box::new(Inline {
        arguments,
        // arguments_matches: None,
    })
}

pub struct Inline {
    arguments: Vec<String>,
    // arguments_matches: Option<ArgMatches>,
}

impl Input for Inline {
    fn argument(&self, name: &str) -> Result<String, String> {
        Err(format!(
            "Argument not found: {}.", name
        ))
    }

    fn with_arguments_matches(&self, _matches: ArgMatches) -> Box<Inline> {
        Box::new(Inline {
            arguments: self.arguments.clone(),
            // arguments_matches: Some(matches),
        })
    }

    fn to_iter(&self) -> Box<dyn Iterator<Item = String>> {
        Box::new(self.arguments.clone().into_iter())
    }
}