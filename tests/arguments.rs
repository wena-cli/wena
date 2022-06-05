use wena::Argument;
use wena::Output;
use wena::Command;
use wena::BufferOutput;
use wena::InlineInput;
use wena::Application;
use wena::Input;

#[test]
fn it_provides_arguments_on_input() {
    let mut app = Application::new("my-app")
        .io(InlineInput::new("my-app", ["add", "my-todo-item"]), BufferOutput::default());

    app.commands([
        Command::<InlineInput, BufferOutput>::new("add")
            .description("Add a new todo")
            .definition([
                Argument::new("todo").required(true),
            ])
            .handler(|app| {
                app.output.writeln(app.input.argument("todo").unwrap());
            }),
    ]).run();

    assert!(app.output.contents.contains("my-todo-item"));
}
