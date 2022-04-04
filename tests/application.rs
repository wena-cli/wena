use wena::output::buffer::*;
use wena::output::*;

#[test]
fn it_has_a_name() {
    let app = wena::new::<Buffer>("my-application", "0.0.1");

    assert_eq!(app.name, "my-application");
}

#[test]
fn it_has_a_version() {
    let app = wena::new::<Buffer>("my-application", "0.0.1");

    assert_eq!(app.version, "0.0.1");
}

#[test]
fn it_has_no_commands_by_default() {
    let app = wena::new::<Buffer>("my-application", "0.0.1");

    assert_eq!(app.commands.len(), 0);
}

#[test]
fn it_may_have_commands() {
    let mut app = wena::new::<Buffer>("my-application", "0.0.1");

    assert_eq!(
        app.command(
            "list",
            "Displays the application commands",
            |_output| {}
        )
        .commands
        .len(),
        1
    );
}

#[test]
fn it_run_commands() {
    let mut app = wena::new::<Buffer>("my-application", "0.0.1");
    let mut output = buffer::new();

    app.command("hello", "Displays hello", |command| {
        command.output.writeln("Hello, world!");
    })
    .run_with_arguments(
        &mut output,
        vec!["my-application".to_string(), "hello".to_string()],
    );

    assert_eq!(output.contents, "Hello, world!\n");
}
