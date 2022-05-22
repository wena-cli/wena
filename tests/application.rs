mod fixtures;

use wena::*;

#[test]

fn it_has_a_name() {
    let app = fixtures::app(vec![], vec![]);

    dbg!(app.output.contents.clone());

    assert_eq!(true, app.output.contents.contains("my-application: 0.0.1"));
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
fn it_may_have_commands() {
    let app = fixtures::app(vec![], vec![
        wena::command("hello", "Displays hello", |_| {})
    ]);

    assert_eq!(1, app.commands.len());
}

#[test]
fn it_run_commands() {
    let app = fixtures::app(vec![], vec![
        wena::command("hello", "Displays hello", |app| {
            app.output.writeln("Hello, world!");
        }),
    ]);
    
    assert_eq!(app.output.contents, "Hello, world!\n");
}
