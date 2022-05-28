use crate::output::Output;

pub struct Console {
    // ..
}

impl Console {
    pub(crate) fn new() -> Self {
        Console {
            // ..
        }
    }
}

impl Output for Console {
    fn writeln(&mut self, string: String) {
        println!("{}", string);
    }
}
