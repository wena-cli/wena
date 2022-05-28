use colored::*;

pub struct Alert {
    bg: Color,
    description: String,
    fg: Color,
    r#type: String,
}

impl Alert {
    pub fn error(description: &str) -> String {
        let alert = Alert {
            bg: Color::Red,
            description: description.to_string(),
            r#type: "ERROR".to_string(),
            fg: Color::White,
        };

        alert.to_string()
    }

    pub fn info(description: &str) -> String {
        let alert = Alert {
            bg: Color::Blue,
            description: description.to_string(),
            r#type: "INFO".to_string(),
            fg: Color::White,
        };

        alert.to_string()
    }

    pub fn warn(description: &str) -> String {
        let alert = Alert {
            bg: Color::Yellow,
            description: description.to_string(),
            r#type: "WARN".to_string(),
            fg: Color::White,
        };

        alert.to_string()
    }
}

impl ToString for Alert {
    fn to_string(&self) -> String {
        let message = format!(" {} ", self.r#type)
            .bold()
            .color(self.fg)
            .on_color(self.bg);

        format!("\n  {} {}\n", message, self.description.clone())
    }
}
