use wena::output::console::*;
use wena::output::*;

fn main() {
    let mut output = console::new();

    wena::new("My application", "0.0.1")
        .command("hello", "Displays hello", |output: &mut Console| {
            output.writeln("Hello, world!");
        })
        .run(&mut output);
}
