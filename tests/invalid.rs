use wena::{Application, BufferOutput, Command, InlineInput, Output};

#[test]
fn it_runs() {
    let mut app = Application::new("my-app").io(
        InlineInput::new("my-app", ["my-invalid-command-name"]),
        BufferOutput::default(),
    );

    let result = app
        .commands([Command::new("add")
            .io::<InlineInput, BufferOutput>()
            .handler(|app| {
                app.output.writeln("Hello, world!");

                Ok(0)
            })])
        .do_run();

    assert_eq!(1, result.unwrap());
    assert!(app.output.contents.contains(
        "Found an argument which wasn't expected or isn't valid in this \
         context"
    ));
}
