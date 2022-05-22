pub mod buffer;
pub mod console;

use colored::*;

pub trait Output {
    fn error(&mut self, string: &str) -> () {
        label("ERROR", string); 
    }

    fn info(&mut self, string: &str) -> () {
        label("INFO", string); 
    }

    fn writeln(&mut self, string: &str);
}

fn label(title: &str, message: &str) -> ()
{
    let label_title = &format!(
        " {} ",
        title.clone().bold().white(),
    );
    
    let label_message = message.clone();

    dbg!(label_title);

    println!("{}", &format!(
        "  {} {}", label_title, label_message
    ));
}
