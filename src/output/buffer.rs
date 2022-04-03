use crate::output::Output;

pub fn new() -> Buffer {
    Buffer {
        contents: "".to_string(),
    }
}

pub struct Buffer {
    pub contents: String,
}

impl Output for Buffer {
    fn writeln(&mut self, string: &str) {
        let _ = &self.contents.push_str(string);
        let _ = &self.contents.push('\n');
    }
}
