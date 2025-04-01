#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Size { width, height }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        let (width, height) = crossterm::terminal::size().unwrap();
        Size {
            width: width as usize,
            height: height as usize,
        }
    }
}
