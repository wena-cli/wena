use crate::output::Output;
use crate::input::Input;
pub use colored::*;

pub mod invalid;
pub mod list;

use crate::application::Application;

pub type Handler<TInput, TOutput> = fn(&mut Application<TInput, TOutput>) -> ();

pub fn new<TInput : Input, TOutput : Output>(name: &str, description: &str, handler: Handler<TInput, TOutput>) -> Command<TInput, TOutput> {
    Command {
        name: name.to_string(),
        description: description.to_string(),
        handler,
    }
}

pub struct Command<TInput : Input + ?Sized, TOutput : Output + ?Sized> {
    pub name: String,
    pub description: String,
    pub handler: Handler<TInput, TOutput>,
}
