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
    fn write(&mut self, string: impl Into<String>) {
        print!("{}", string.into());
    }
}
