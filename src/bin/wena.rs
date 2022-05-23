use wena::*;

fn main() {
    wena::app("wena")
        .version("0.0.1")
        .command("hello", |command| {
            command
                .argument("name", |argument| argument)
                .handler(|application| {
                    let name = application.input.argument("name");

                    application
                        .output
                        .info(format!("Hello, {}!", name).as_str());
                })
        })
        .run();
}
