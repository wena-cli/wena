use regex::Regex;

use crate::output::Output;

pub struct BufferOutput {
    pub contents: String,
}

impl BufferOutput {
    #[allow(dead_code)]
    pub fn new(contents: String) -> Self {
        BufferOutput { contents }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Self::new(String::from(""))
    }
}

impl Output for BufferOutput {
    fn write(&mut self, string: impl Into<String>) {
        let string = string.into();

        let strip_sequeces_regex =
            Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
        let strip_sequeces_string =
            strip_sequeces_regex.replace_all(string.as_str(), "");

        self.contents.push_str(&strip_sequeces_string);
    }
}
