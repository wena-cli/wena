use crate::output::Output;

pub fn new() -> Box<Buffer> {
    Box::new(Buffer {
        // contents: String::from(""),
    })
}

pub struct Buffer {
    // contents: String,
}

impl Output for Buffer {
    fn writeln(&self, _string: &str) {
        // self.contents.push_str(string);
        // self.contents.push_str("\n");
    }
}
