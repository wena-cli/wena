mod argument;
mod argv;
mod inline;
mod null;

use std::fmt::Debug;
use std::str::FromStr;

pub use argument::Argument;
pub use argv::ArgvInput;
use clap::{ArgMatches, Command, Error, Result};
pub use inline::InlineInput;
pub use null::NullInput;

pub trait Input {
    fn argument<TValue: FromStr>(&self, name: &str) -> Option<TValue>
    where
        <TValue as FromStr>::Err: Debug,
    {
        if let Ok(matches) = &self.matches() {
            if let Some((_, sub_matches)) = matches.subcommand() {
                let value = sub_matches.value_of(name);

                if let Some(value) = value {
                    return Some(value.to_string().parse::<TValue>().unwrap());
                }

                return None;
            }
        }

        None
    }

    fn find_matches(&mut self, command: Command);

    fn matches(&self) -> Result<&ArgMatches, &Error>;
}
