use wena::Output;
use wena::Command;
use wena::BufferOutput;
use wena::InlineInput;
use wena::Application;

#[test]
fn it_runs() {
    let mut app = Application::new("my-app")
        .io(InlineInput::new("my-app", ["add"]), BufferOutput::default());

    app.commands([
        Command::<InlineInput, BufferOutput>::new("add").handler(|app| {
            app.output.writeln("Hello, world!");
        }),
    ]).run();

    assert!(app.output.contents.contains("Hello, world!"));
}
