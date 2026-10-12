#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geometry {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Geometry {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
