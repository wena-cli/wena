use crate::commands::Command;
use crate::output::Output;
use crate::input::Input;

pub fn new<TInput : Input, TOutput : Output>(options: Options<TInput, TOutput>) -> Application<TInput, TOutput> {

    Application {
        name: options.name.to_string(),
        version: options.version.to_string(),
        commands: options.commands,
        input: options.input,
        output: options.output,
    }
}

pub struct Application<TInput : Input + ?Sized, TOutput : Output + ?Sized> {
    pub name: String,
    pub version: String,
    pub commands: Vec<Command<TInput, TOutput>>,
    pub input: Box<TInput> ,
    pub output: Box<TOutput>,
}

pub struct Options<'a, TInput: Input, TOutput : Output> {
    pub name: &'a str,
    pub version: &'a str,
    pub commands: Vec<Command<TInput, TOutput>>,
    pub input: Box<TInput>,
    pub output: Box<TOutput>,
}

impl<TInput: Input, TOutput: Output> Application<TInput, TOutput> {
    pub fn run(&mut self) -> &Application<TInput, TOutput> {
        self.run_with_arguments()
    }

    pub fn run_with_arguments(&mut self) -> &Application<TInput, TOutput> {
        crate::runner::run::<TInput, TOutput>(self)
    }
}
