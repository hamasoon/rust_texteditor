#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}
