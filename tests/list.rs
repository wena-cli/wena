use wena::Command;
use wena::BufferOutput;
use wena::InlineInput;
use wena::Application;

#[test]
fn it_runs() {
    let arguments: [String; 0] = [];

    let mut app = Application::new("my-app")
        .io(InlineInput::new("my-app", arguments), BufferOutput::default());

    app.commands([
        Command::new("add")
            .io::<InlineInput, BufferOutput>()
            .description("Add a new todo"),
    ]).run();

    assert!(app.output.contents.contains("add"));
    assert!(app.output.contents.contains("Add a new todo"));

}
