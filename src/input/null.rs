use crate::input::{ Input };

pub fn new() -> Box<Null> {
    Box::new(Null {})
}

pub struct Null {
    // ..
}

impl Input for Null {
    fn argument(&self, _name: &str) -> Result<String, String> {
        Err("Argument not found".to_string())
    }

    fn to_iter(&self) -> Box<dyn Iterator<Item = String>> {
        Box::new(std::iter::empty())
    }
}