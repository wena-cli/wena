mod assertions;
mod fixtures;

use assertions::*;
use wena::*;

#[test]
fn it_runs() {
    let app = fixtures::app(
        vec!["invalid command".to_string()],
        vec![wena::command("hello").handler(|app| {
            app.output.writeln("Hello, world!");
        })],
    );

    assert_output(app, "Command not found.");
}
