use wena::{Application, BufferOutput, Command, InlineInput, Output};

#[test]
fn it_has_a_name() {
    let app = Application::new("my-app");

    assert_eq!("my-app", app.name);
}

#[test]
fn it_has_a_default_version() {
    let version = Application::new("my-app").version;

    assert_eq!("1.0.0", version);
}

#[test]
fn it_has_version() {
    let version = Application::new("my-app")
        .version("1.0.1")
        .version
        .to_string();

    assert_eq!("1.0.1", version);
}

#[test]
fn it_has_no_commands_by_default() {
    let app = Application::new("my-app");

    assert_eq!(app.commands.len(), 0);
}

#[test]
fn it_can_have_commands() {
    let mut app = Application::new("my-app")
        .io(InlineInput::new("todo", ["add"]), BufferOutput::default());

    app.commands([Command::<InlineInput, BufferOutput>::new("add").handler(
        |app| {
            app.output.writeln("Hello, world!");
        },
    )])
    .run();

    dbg!(app.output.contents.clone());

    assert!(app.output.contents.contains("Hello, world!"));
}
