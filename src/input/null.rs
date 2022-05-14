use crate::input::{ Input };

pub fn new() -> Box<dyn Input> {
    Box::new(Null {})
}

pub struct Null {
    // ..
}

impl Input for Null {
    fn argument(&self, _name: &str) -> Result<String, String> {
        Err("Argument not found".to_string())
    }
}