mod argument;
mod argv;
mod inline;
mod null;

pub use argument::Argument;
pub use argv::ArgvInput;
use clap::{ArgMatches, Command, Error, Result};
pub use inline::InlineInput;
pub use null::NullInput;

pub trait Input {
    fn argument(&self, name: &str) -> Option<&str> {
        if let Ok(matches) = &self.matches() {
            if let Some((_, sub_matches)) = matches.subcommand() {
                return sub_matches.value_of(name);
            }
        }

        None
    }

    fn find_matches(&mut self, command: Command);

    fn matches(&self) -> Result<&ArgMatches, &Error>;
}
