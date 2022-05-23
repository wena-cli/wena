use crate::input::Input;
use crate::Output;
use clap::Arg;
pub use colored::*;

pub mod invalid;
pub mod list;

use crate::application::Application;

pub type Handler<TInput, TOutput> = fn(&mut Application<TInput, TOutput>) -> ();

pub fn new<TInput: Input, TOutput: Output>(name: &str) -> Command<TInput, TOutput> {
    Command {
        arguments: vec![],
        description: String::from(""),
        handler: |_| {},
        name: name.to_string(),
    }
}

pub struct Command<TInput: Input + ?Sized, TOutput: Output + ?Sized> {
    pub name: String,
    pub arguments: Vec<Arg<'static>>,
    pub description: String,
    pub handler: Handler<TInput, TOutput>,
}

impl<TInput: Input + ?Sized, TOutput: Output + ?Sized> Command<TInput, TOutput> {
    pub fn argument(
        mut self,
        name: &'static str,
        resolver: impl Fn(Arg) -> Arg,
    ) -> Command<TInput, TOutput> {
        let argument = Arg::new(name);

        self.arguments.push(resolver(argument));

        self
    }

    pub fn description(mut self, value: &str) -> Self {
        self.description = value.to_string();

        self
    }

    pub fn handler(mut self, value: Handler<TInput, TOutput>) -> Self {
        self.handler = value;

        self
    }
}
