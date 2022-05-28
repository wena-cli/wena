mod argv;
mod inline;
mod null;

pub use argv::Argv;
use clap::ArgMatches;
pub use inline::Inline;
pub use null::Null;

pub trait Input<TInput: ?Sized = Self> {
    fn argument(&self, string: &str) -> &str;

    fn to_iter(&self) -> Box<dyn Iterator<Item = String>>;

    fn with_arguments_matches(&self, arguments: ArgMatches) -> Box<TInput>;
}
