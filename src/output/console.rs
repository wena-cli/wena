use crate::output::Output;

pub fn new() -> Box<dyn Output> {
    Box::new(Console {})
}

pub struct Console {
    // ..
}

impl Output for Console {
    fn writeln(&self, string: &str) {
        println!("{}",  string);
    }
}