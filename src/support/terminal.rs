use terminal_size::terminal_size;

pub struct Terminal {
    width: usize,
}

impl Terminal {
    pub fn new(width: usize) -> Self {
        Self { width }
    }

    pub fn default() -> Self {
        let sizes = terminal_size();

        if let Some((width, _)) = sizes {
            Self::new(width.0 as usize)
        } else {
            Self::new(80)
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
