use clap::ArgMatches;

use crate::input::Input;

pub struct Null {
    // ..
}

impl Null {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Null {
            // ..
        }
    }
}

impl Input for Null {
    fn argument(&self, name: &str) -> &str {
        panic!("Argument not found: {}.", name);
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
