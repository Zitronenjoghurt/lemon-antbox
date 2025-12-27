#[derive(Debug, Default)]
pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub tribe: u8,
    pub angle: f32,
    pub has_food: bool,
}

impl Ant {
    pub fn color_rgba(&self) -> [u8; 4] {
        [165, 102, 47, 255]
    }
}
