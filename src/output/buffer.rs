use crate::output::Output;
use regex::Regex;

pub fn new() -> Box<Buffer> {
    Box::new(Buffer {
        contents: String::from(""),
    })
}

pub struct Buffer {
    pub contents: String,
}

impl Output for Buffer {
    fn writeln(&mut self, string: &str) {
        let strip_sequeces_regex = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
        let strip_sequeces_string = strip_sequeces_regex.replace_all(string, "");

        self.contents.push_str(&strip_sequeces_string);
        self.contents.push('\n');
    }
}
