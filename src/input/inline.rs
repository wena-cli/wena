use clap::ArgMatches;

use crate::input::Input;

pub struct Inline {
    arguments: Vec<String>,
    arguments_matches: Option<ArgMatches>,
}

impl Inline {
    #[allow(dead_code)]
    pub fn new(binary_name: String, arguments: Vec<String>) -> Self {
        let append = vec![binary_name];

        Inline {
            arguments: [&append[..], &arguments[..]].concat(),
            arguments_matches: None,
        }
    }
}

impl Input for Inline {
    fn argument(&self, name: &str) -> &str {
        if let Some(arguments_matches) = &self.arguments_matches {
            if let Some(value) = arguments_matches.value_of(name) {
                return value;
            }
        }

        panic!("Argument not found: {}.", name);
    }

    fn with_arguments_matches(&self, matches: ArgMatches) -> Box<Inline> {
        Box::new(Inline {
            arguments: self.arguments.clone(),
            arguments_matches: Some(matches),
        })
    }

    fn to_iter(&self) -> Box<dyn Iterator<Item = String>> {
        Box::new(self.arguments.clone().into_iter())
    }
}
