pub use colored::*;

pub mod invalid;
pub mod list;

use crate::application::Application;

pub type Handler = fn(&Application) -> ();

pub fn new(name: &str, description: &str, handler: Handler) -> Command {
    Command {
        name: name.to_string(),
        description: description.to_string(),
        handler,
    }
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub handler: Handler,
}
