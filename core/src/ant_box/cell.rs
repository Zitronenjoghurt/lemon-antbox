#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {}

impl Cell {
    pub fn color_rgba(&self) -> [u8; 4] {
        [0, 0, 0, 255]
    }
}
