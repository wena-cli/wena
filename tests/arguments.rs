mod assertions;
mod fixtures;

use assertions::*;

#[test]
fn it_reads_arguments() {
    let app = fixtures::app(
        vec!["hello".to_string(), "nuno".to_string()],
        vec![
            wena::command("hello")
                .description("Displays hello")
                .argument("name", |argument| argument.required(true))
                .handler(|app| {
                    let name = app.input.argument("name");

                    app.output.writeln(format!("Hello, {}!", name));
                }),
        ],
    );

    assert_output(app, "Hello, nuno!");
}
