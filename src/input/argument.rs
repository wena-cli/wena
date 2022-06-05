use clap::Arg;

pub struct Argument {
    // ..
}

impl Argument {
    pub fn new(name: &str) -> Arg {
        Arg::new(name)
    }
}
