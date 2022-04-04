use wena::output::console::*;
use wena::output::*;

fn main() {
    let mut output = console::new();

    wena::new::<Console>("My application", "0.0.1")
        .command("hello", "Displays hello", |command| {
            command.output.writeln("Hello, world!");
        })
        .run(&mut output);
}
