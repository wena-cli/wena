use regex::Regex;
use crate::output::Output;
use crate::input::Input;
pub use colored::*;

pub mod invalid;
pub mod list;

use crate::application::Application;

pub type Handler<TInput, TOutput> = fn(&mut Application<TInput, TOutput>) -> ();

pub fn new<TInput : Input, TOutput : Output>(signature: &str, description: &str, handler: Handler<TInput, TOutput>) -> Command<TInput, TOutput> {

    let name = Regex::new("([^\\s]+)")
        .unwrap()
        .captures(signature)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string();

    let tokens = Regex::new("\\s+")
        .unwrap()
        .split(signature)
        .map(|argument| argument.to_string())
        .collect::<Vec<String>>();

    Command {
        name,
        description: description.to_string(),
        handler,
        tokens,
    }
}

pub struct Command<TInput : Input + ?Sized, TOutput : Output + ?Sized> {
    pub name: String,
    pub description: String,
    pub handler: Handler<TInput, TOutput>,
    pub tokens: Vec<String>,
}
