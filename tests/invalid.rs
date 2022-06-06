use wena::{Application, BufferOutput, Command, InlineInput, Output};

#[test]
fn it_runs() {
    let mut app = Application::new("my-app")
        .io(InlineInput::new("my-app", ["add"]), BufferOutput::default());

    let result = app
        .commands([Command::new("add")
            .io::<InlineInput, BufferOutput>()
            .handler(|app| {
                app.output.writeln("Hello, world!");

                Ok(1)
            })])
        .do_run();

    assert_eq!(1, result.unwrap());
    assert!(app.output.contents.contains("Hello, world!"));
}
