use clap::ArgMatches;

pub mod argv;
pub mod inline;
pub mod null;

pub trait Input<TInput: ?Sized = Self> {
    fn argument(&self, string: &str) -> &str;

    fn to_iter(&self) -> Box<dyn Iterator<Item = String>>;

    fn with_arguments_matches(&self, arguments: ArgMatches) -> Box<TInput>;
}
