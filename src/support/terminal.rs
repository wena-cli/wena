use terminal_size::terminal_size;

pub struct Terminal {
    width: usize,
}

impl Terminal {
    pub fn new(width: usize) -> Self {
        Self { width }
    }

    pub fn default() -> Self {
        let (width, _) = terminal_size().unwrap();

        Self::new(width.0 as usize)
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
