use wena::input::Input;
use wena::output::Output;

fn main() {
    wena::app("wena")
        .version("0.0.1")
        .command("hello", |command| {
            command
                .argument("name", |argument| argument)
                .handler(|app| {
                    let name = app.input.argument("name");

                    app.output.info(format!("Hello, {}!", name).as_str());
                })
        })
        .run();
}
