mod assertions;
mod fixtures;

use assertions::*;
use wena::*;

#[test]
fn it_runs() {
    let app = fixtures::app(
        vec!["my-other-hello-command".to_string()],
        vec![wena::command("hello", "Displays hello", |app| {
            app.output.writeln("Hello, world!");
        })],
    );

    assert_output(app, "Command not found.");
}
