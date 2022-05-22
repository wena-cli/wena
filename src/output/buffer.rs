use crate::output::Output;

pub fn new() -> Box<Buffer> 
{
    Box::new(Buffer {
        contents: String::from(""),
    })
}

pub struct Buffer {
    pub contents: String,
}

impl Output for Buffer {
    fn writeln(&mut self, string: &str) {
        self.contents.push_str(string);
        self.contents.push('\n');
    }
}
