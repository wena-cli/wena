use wena::{Alert, *};

fn main() {
    wena::app("wena")
        .version("0.0.1")
        .command("hello", |command| {
            command
                .argument("name", |argument| argument)
                .handler(|app| {
                    let name = app.input.argument("name");

                    app.output.writeln(Alert::info(
                        format!("Hello, {}!", name).as_str(),
                    ));
                })
        })
        .run();
}
