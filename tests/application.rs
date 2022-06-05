mod assertions;
mod fixtures;

use assertions::*;
use wena::*;

#[test]
fn it_has_a_name() {
    let app = fixtures::app(vec![], vec![]);

    assert_output(app, "my-application");
}

#[test]
fn it_has_a_version() {
    let app = fixtures::app(vec![], vec![]);

    assert_eq!(app.version, "0.0.1");
}

#[test]
fn it_has_no_commands_by_default() {
    let app = fixtures::app(vec![], vec![]);

    assert_eq!(app.commands.len(), 0);
}

#[test]
fn it_can_have_commands() {
    let app =
        fixtures::app(vec![], vec![wena::command("hello").handler(|_| {})]);

    assert_eq!(1, app.commands.len());
}

#[test]
fn it_run_commands() {
    let app = fixtures::app(
        vec!["hello".to_string()],
        vec![command("hello").handler(|app| {
            app.output.writeln("Hello, world!".to_string());
        })],
    );

    assert_eq!(app.output.contents, "Hello, world!\n");
}
