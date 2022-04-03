use crate::output::Output;

pub fn new() -> Console {
    Console {}
}

pub struct Console {
    // ..
}

impl Output for Console {
    fn writeln(&mut self, string: &str) {
        println!("{}", string);
    }
}
