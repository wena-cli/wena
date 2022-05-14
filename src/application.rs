use crate::commands::Command;
use crate::output;
use crate::output::Output;
use crate::input;
use crate::input::Input;

pub fn new(options: Options) -> Application {
    Application {
        name: options.name.to_string(),
        version: options.version.to_string(),
        commands: options.commands.unwrap_or_default(),
        input: options.input.unwrap_or_else(input::null::new),
        output: options.output.unwrap_or_else(output::console::new),
    }
}

pub struct Application {
    pub name: String,
    pub version: String,
    pub commands: Vec<Command>,
    pub input: Box<dyn Input> ,
    pub output: Box<dyn Output>,
}

pub struct Options<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub commands: Option<Vec<Command>>,
    pub input: Option<Box<dyn Input>>,
    pub output: Option<Box<dyn Output>>,
}

impl Application {
    pub fn run(&self) {
        self.run_with_arguments(std::env::args().collect());
    }

    pub fn run_with_arguments(&self, arguments: Vec<String>) {
        crate::runner::run(self, arguments);
    }
}
