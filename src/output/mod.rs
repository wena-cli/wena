pub mod buffer;
pub mod console;

pub trait Output {
    fn writeln(&self, string: &str);
}
