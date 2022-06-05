use crate::commands::Command;
use crate::input::Input;
use crate::output::Output;
use crate::{ArgvInput, ConsoleOutput};

pub struct Application<TInput: Input + ?Sized, TOutput: Output + ?Sized> {
    pub name: String,
    pub version: String,
    pub commands: Vec<Command<TInput, TOutput>>,
    pub input: Box<TInput>,
    pub output: Box<TOutput>,
}

impl Application<ArgvInput, ConsoleOutput> {
    pub fn new(
        name: impl Into<String>,
    ) -> Application<ArgvInput, ConsoleOutput> {
        Self {
            name: name.into(),
            version: "1.0.0".to_string(),
            commands: vec![],
            input: Box::new(ArgvInput::new()),
            output: Box::new(ConsoleOutput::new()),
        }
    }
}

impl<TInput: Input, TOutput: Output> Application<TInput, TOutput> {
    pub fn io<TGivenInput: Input, TGivenOutput: Output>(
        &self,
        input: TGivenInput,
        output: TGivenOutput,
    ) -> Application<TGivenInput, TGivenOutput> {
        Application {
            name: self.name.clone(),
            version: self.version.clone(),
            commands: Vec::new(),
            input: Box::new(input),
            output: Box::new(output),
        }
    }

    pub fn commands(
        &mut self,
        commands: impl IntoIterator<Item = Command<TInput, TOutput>>,
    ) -> &mut Application<TInput, TOutput> {
        for command in commands {
            self.commands.push(command);
        }

        self
    }

    pub fn name(&mut self, version: &str) -> &mut Application<TInput, TOutput> {
        self.version = version.to_string();

        self
    }

    pub fn version(
        &mut self,
        version: &str,
    ) -> &mut Application<TInput, TOutput> {
        self.version = version.to_string();

        self
    }

    pub fn run(&mut self) -> &Application<TInput, TOutput> {
        crate::runner::run::<TInput, TOutput>(self)
    }
}
