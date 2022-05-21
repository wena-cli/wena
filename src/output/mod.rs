pub mod buffer;
pub mod console;

pub trait Output {
    fn writeln(&mut self, string: &str);
}
