use crate::commands::Command;
use crate::commands::Handler;
use crate::output::Output;

pub fn new<TOutput: Output>(name: &str, version: &str) -> Application<TOutput> {
    Application {
        name: name.to_string(),
        version: version.to_string(),
        commands: Vec::new(),
    }
}

pub struct Application<TOutput: Output> {
    pub name: String,
    pub version: String,
    pub commands: Vec<Command<TOutput>>,
}

impl<TOutput: Output> Application<TOutput> {
    pub fn command(
        &mut self,
        name: &str,
        description: &str,
        callback: Handler<TOutput>,
    ) -> &mut Application<TOutput> {
        let command = crate::commands::new::<TOutput>(name, description, callback);

        let _ = &self.commands.push(command);

        self
    }

    pub fn run(&self, output: &mut TOutput) {
        self.run_with_arguments(output, std::env::args().collect());
    }

    pub fn run_with_arguments(&self, output: &mut TOutput, arguments: Vec<String>) {
        crate::runner::run::<TOutput>(self, arguments, output);
    }
}
