#[derive(Debug)]
pub struct Ant {
    pub x: u16,
    pub y: u16,
}

impl Ant {
    pub fn color_rgba(&self) -> [u8; 4] {
        [165, 102, 47, 255]
    }
}
