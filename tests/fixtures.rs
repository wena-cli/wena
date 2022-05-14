use wena::application;
use wena::input;
use wena::output;

pub fn app(commands: Vec<wena::commands::Command>) -> application::Application {
    let input = input::null::new();
    let output = output::buffer::new();

    wena::app_with({ application::Options {
        name: "my-application",
        version: "0.0.1",
        commands: Some(commands),
        input: Some(input),
        output: Some(output),
    }})
}
