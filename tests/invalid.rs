mod fixtures;

use wena::*;

#[test]
fn it_runs() {
    let app = fixtures::app(vec!["hello".to_string()], vec![
        wena::command("hello", "Displays hello", |app| {
            app.output.writeln("Hello, world!");
        }),
    ]);

    assert_eq!(app.output.contents, "Command not found.\n");
}
