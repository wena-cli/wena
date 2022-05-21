use wena::application;
use wena::commands::Command;
use wena::input;
use wena::output;

pub fn app(input: Vec<String>, commands: Vec<Command>) {
    
}


pub fn app(input: Box<dyn input::Input>, output: Box<dyn output::Output>, commands: Vec<commands::Command>) -> application::Application {
    let
    
    
    wena::from({ application::Options {
        name: "my-application",
        version: "0.0.1",
        commands: commands,
        input: input,
        output: output,
    }})
}
