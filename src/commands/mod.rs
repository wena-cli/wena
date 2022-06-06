mod invalid;
mod list;

use clap::Arg;
pub use invalid::InvalidCommandFactory;
pub use list::ListCommandFactory;

use crate::application::Application;
use crate::input::Input;
use crate::output::Output;

pub struct Command<TInput: Input + ?Sized, TOutput: Output + ?Sized> {
    pub name: String,
    pub definition: Vec<Arg<'static>>,
    pub description: String,
    pub handler: Handler<TInput, TOutput>,
}

impl<TInput: Input + ?Sized, TOutput: Output + ?Sized>
    Command<TInput, TOutput>
{
    pub fn new(name: impl Into<String>) -> Command<TInput, TOutput> {
        Command {
            definition: vec![],
            description: String::from(""),
            handler: |_| {},
            name: name.into(),
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn definition(
        mut self,
        arguments: impl IntoIterator<Item = Arg<'static>>,
    ) -> Self {
        for argument in arguments {
            self.definition.push(argument);
        }

        self
    }

    pub fn handler(mut self, value: Handler<TInput, TOutput>) -> Self {
        self.handler = value;

        self
    }
}

type Handler<TInput, TOutput> = fn(&mut Application<TInput, TOutput>) -> ();
