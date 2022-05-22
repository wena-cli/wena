use crate::input::Input;
use clap::ArgMatches;

pub fn new() -> Box<Null> {
    Box::new(Null {
        // ..
    })
}

pub struct Null {
    // ..
}

impl Input for Null {
    fn argument(&self, name: &str) -> Result<String, String> {
        Err(format!("Argument not found: {}.", name))
    }

    fn with_arguments_matches(&self, _matches: ArgMatches) -> Box<Null> {
        Box::new(Null {
            // ..
        })
    }

    fn to_iter(&self) -> Box<dyn Iterator<Item = String>> {
        Box::new(std::iter::empty())
    }
}
