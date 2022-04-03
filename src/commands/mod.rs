pub mod invalid;
pub mod list;

use crate::output::Output;

pub type Handler<TOutput> = fn(&mut TOutput) -> ();

pub fn new<TOutput: Output>(
    name: &str,
    description: &str,
    handler: Handler<TOutput>,
) -> Command<TOutput> {
    Command {
        name: name.to_string(),
        description: description.to_string(),
        handler,
    }
}

pub struct Command<TOutput: Output> {
    pub name: String,
    pub description: String,
    pub handler: Handler<TOutput>,
}
