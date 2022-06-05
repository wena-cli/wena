use wena::commands::Command;
use wena::input::Inline;
use wena::output::Buffer;
use wena::{Application, ApplicationOptions};

#[allow(dead_code)]
pub fn app(
    input: Vec<String>,
    commands: Vec<Command<Inline, Buffer>>,
) -> Application<Inline, Buffer> {
    let mut app = Application::new(ApplicationOptions {
        name: "my-application",
        version: "0.0.1",
        commands,
        input: Box::new(Inline::new("my-application".to_string(), input)),
        output: Box::new(Buffer::default()),
    });

    app.run();

    app
}
