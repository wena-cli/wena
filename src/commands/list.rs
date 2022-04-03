use crate::commands::Command;
use crate::output::Output;

pub fn new<TOutput: Output>() -> Command<TOutput> {
    crate::commands::new::<TOutput>("list", "Displays the application commands", |output| {
        output.writeln("Listing commands.");
    })
}
