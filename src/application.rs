use crate::commands::{self, Command};
use crate::input::Input;
use crate::Output;

pub fn new<TInput: Input, TOutput: Output>(
    options: Options<TInput, TOutput>,
) -> Application<TInput, TOutput> {
    Application {
        name: options.name.to_string(),
        version: options.version.to_string(),
        commands: options.commands,
        input: options.input,
        output: options.output,
    }
}

pub struct Application<TInput: Input + ?Sized, TOutput: Output + ?Sized> {
    pub name: String,
    pub version: String,
    pub commands: Vec<Command<TInput, TOutput>>,
    pub input: Box<TInput>,
    pub output: Box<TOutput>,
}

pub struct Options<'a, TInput: Input, TOutput: Output> {
    pub name: &'a str,
    pub version: &'a str,
    pub commands: Vec<Command<TInput, TOutput>>,
    pub input: Box<TInput>,
    pub output: Box<TOutput>,
}

impl<TInput: Input, TOutput: Output> Application<TInput, TOutput> {
    pub fn command(
        &mut self,
        name: &str,
        resolver: impl Fn(Command<TInput, TOutput>) -> Command<TInput, TOutput>,
    ) -> &mut Application<TInput, TOutput> {
        let command = commands::new::<TInput, TOutput>(name);

        self.commands.push(resolver(command));

        self
    }

    pub fn version(&mut self, version: &str) -> &mut Application<TInput, TOutput> {
        self.version = version.to_string();

        self
    }

    pub fn run(&mut self) -> &Application<TInput, TOutput> {
        crate::runner::run::<TInput, TOutput>(self)
    }
}
