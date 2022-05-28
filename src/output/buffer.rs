use regex::Regex;

use crate::output::Output;

pub struct Buffer {
    pub contents: String,
}

impl Buffer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Buffer {
            contents: String::from(""),
        }
    }
}

impl Output for Buffer {
    fn writeln(&mut self, string: String) {
        let strip_sequeces_regex =
            Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
        let strip_sequeces_string =
            strip_sequeces_regex.replace_all(string.as_str(), "");

        self.contents.push_str(&strip_sequeces_string);
        self.contents.push('\n');
    }
}
