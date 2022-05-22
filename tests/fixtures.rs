use wena::application;
use wena::application::Application;
use wena::commands::Command;
use wena::input::inline;
use wena::input::inline::Inline;
use wena::output::buffer;
use wena::output::buffer::Buffer;

pub fn app(
    input: Vec<String>,
    commands: Vec<Command<Inline, Buffer>>,
) -> Application<Inline, Buffer> {
    let mut app = application::new(application::Options {
        name: "my-application",
        version: "0.0.1",
        commands,
        input: inline::new("my-application".to_string(), input),
        output: buffer::new(),
    });

    app.run();

    app
}
