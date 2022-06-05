use terminal_size::terminal_size;

pub struct Terminal
{
    width: usize,
    height: usize,
}

impl Terminal
{
    pub fn new(width: usize, height: usize) -> Self
    {
        Self
        {
            width,
            height,
        }
    }

    pub fn default() -> Self
    {
        let (width, height) = terminal_size().unwrap();

        Self::new(width.0 as usize, height.0 as usize)
    }


    pub fn width(&self) -> usize
    {
        self.width
    }

    pub fn height(&self) -> usize
    {
        self.height
    }
}