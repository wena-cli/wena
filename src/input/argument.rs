use clap::Arg;

pub struct Argument {
    // ..
}

impl Argument {
    #![allow(clippy::new_ret_no_self)]
    pub fn new(name: &str) -> Arg {
        Arg::new(name)
    }
}
