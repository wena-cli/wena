pub use colored::{ Color, ColoredString };

impl Style {
    fn padding(mut self, padding: i16) -> ColoredString {
        let mut result = ColoredString::new();
        let mut padding = padding;

        while padding > 0 {
            result.push_str(" ");
            padding -= 1;
        }

        result.push_str(self);

        result
    }
}