pub mod buffer;
pub mod console;

use colored::*;

pub trait Output {
    fn error(&mut self, string: &str)
    where
        Self: Sized,
    {
        label(self, "ERROR", string);
    }

    fn info(&mut self, string: &str)
    where
        Self: Sized,
    {
        label(self, "INFO", string);
    }

    fn writeln(&mut self, string: &str);
}

fn label(output: &mut dyn Output, label: &str, message: &str) {
    let label_title = &format!(" {} ", label.clone().bold().white(),);

    let label_message = message.clone();

    output.writeln(&format!("  {} {}", label_title, label_message));
}
