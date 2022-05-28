use colored::*;

use crate::Output;

pub struct Alert {
    pub r#type: String,
    pub message: String,
    pub fg: Color,
    pub bg: Color,
}

impl Alert {
    pub fn new(r#type: &str, message: &str, fg: Color, bg: Color) -> Self {
        Alert {
            r#type: r#type.to_string(),
            message: message.to_string(),
            fg,
            bg,
        }
    }

    pub fn writeln(&self, output: &mut dyn Output) {
        let message = format!(" {} ", self.r#type)
            .bold()
            .color(self.fg)
            .on_color(self.bg);

        output.writeln(String::from("").as_str());
        output.writeln(&format!("  {} {}", message, self.message.clone()));
        output.writeln(String::from("").as_str());
    }
}
