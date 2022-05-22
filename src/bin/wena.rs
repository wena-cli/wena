use wena::*;

fn main() {
    wena::app(
        "my-application",
        "0.0.1",
        vec![wena::command("hello", "Displays hello", |app| {
            app.output.info("Hello Wolrd");
        })],
    )
    .run();
}
