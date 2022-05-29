mod buffer;
mod console;

pub use buffer::Buffer;
pub use console::Console;

pub trait Output { 
    fn write(&mut self, string: impl Into<String>);
    
    fn writeln(&mut self, string: impl Into<String>) {
        self.write(string);
        self.new_line();
    }

    fn new_line(&mut self) {
        self.write("\n");
    }
}
