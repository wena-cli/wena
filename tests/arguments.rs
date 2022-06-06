use wena::{
    Application, Argument, BufferOutput, Command, InlineInput, Input, Output,
};

#[test]
fn it_provides_arguments_on_input() {
    let mut app = Application::new("my-app").io(
        InlineInput::new("my-app", ["add", "my-todo-item"]),
        BufferOutput::default(),
    );

    let result = app
        .commands([Command::new("add")
            .io::<InlineInput, BufferOutput>()
            .description("Add a new todo")
            .definition([Argument::new("todo").required(true)])
            .handler(|app| {
                app.output
                    .writeln(app.input.argument::<String>("todo").unwrap());

                Ok(0)
            })])
        .do_run();

    assert_eq!(0, result.unwrap());
    assert!(app.output.contents.contains("my-todo-item"));
}
