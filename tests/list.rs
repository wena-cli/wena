use wena::{Application, BufferOutput, Command, InlineInput};

#[test]
fn it_runs() {
    let arguments: [String; 0] = [];

    let mut app = Application::new("my-app").io(
        InlineInput::new("my-app", arguments),
        BufferOutput::default(),
    );

    let result = app
        .commands([Command::new("add")
            .io::<InlineInput, BufferOutput>()
            .description("Add a new todo")])
        .do_run();

    assert_eq!(0, result.unwrap());
    assert!(app.output.contents.contains("add"));
    assert!(app.output.contents.contains("Add a new todo"));
}
