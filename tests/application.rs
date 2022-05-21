mod fixtures;

use wena::input;
use wena::output;

#[test]

fn it_has_a_name() {
    let app = fixtures::app(
        input::null::new(),
        output::buffer::new(),
        vec![],
    );

    assert_eq!(app.name, "my-application");
}

#[test]
fn it_has_a_version() {
    let app = fixtures::app(
        input::null::new(),
        output::buffer::new(),
        vec![],
    );

    assert_eq!(app.version, "0.0.1");
}

#[test]
fn it_has_no_commands_by_default() {
    let app = fixtures::app(
        input::null::new(),
        output::buffer::new(),
        vec![],
    );

    assert_eq!(app.commands.len(), 0);
}

#[test]
fn it_may_have_commands() {
    let app = fixtures::app(
        input::null::new(),
        output::buffer::new(),
        vec![
            wena::command("hello", "Displays hello", |_| {}),
        ],
    );

    assert_eq!(1, app.commands.len());
}

#[test]
fn it_run_commands() {
    let app = fixtures::app(
        input::null::new(),
        output::buffer::new(),
        vec![
            wena::command("hello", "Displays hello", |app| {
                app.output.writeln("Hello, world!");
            }),
        ]
    ).run_with_arguments(
        vec!["my-application".to_string(), "hello".to_string()],
    );
    
    assert_eq!(app.output.contents, "Hello, world!\n");
}
