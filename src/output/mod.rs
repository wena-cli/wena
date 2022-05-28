mod buffer;
mod console;

pub use buffer::Buffer;
pub use console::Console;

pub trait Output {
    fn writeln(&mut self, string: String)
    where
        Self: Sized;
}
