use crate::output::Output;

pub struct ConsoleOutput {
    // ..
}

impl ConsoleOutput {
    pub(crate) fn new() -> Self {
        ConsoleOutput {
            // ..
        }
    }
}

impl Output for ConsoleOutput {
    fn write(&mut self, string: impl Into<String>) {
        print!("{}", string.into());
    }
}
