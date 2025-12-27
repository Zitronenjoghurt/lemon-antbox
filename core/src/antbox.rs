#[derive(Debug)]
pub struct AntBox {
    height: u32,
    width: u32,
}

impl AntBox {
    pub fn new(height: u32, width: u32) -> Self {
        Self { height, width }
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[120, 120, 120, 255]);
        }
    }
}
