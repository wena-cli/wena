pub mod alerts;
pub mod buffer;
pub mod console;

use crate::output::alerts::Alert;
use colored::*;

pub trait Output {
    fn error(&mut self, string: &str)
    where
        Self: Sized,
    {
        Alert::new("ERROR", string, Color::White, Color::Red).writeln(self);
    }

    fn info(&mut self, string: &str)
    where
        Self: Sized,
    {
        Alert::new("INFO", string, Color::White, Color::Blue).writeln(self);
    }

    fn writeln(&mut self, string: &str);
}
