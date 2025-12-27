use crate::antbox::AntBox;

pub struct Simulation {
    ant_box: AntBox,
    paused: bool,
}

impl Simulation {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            ant_box: AntBox::new(height, width),
            paused: false,
        }
    }

    pub fn step(&mut self) {
        if self.is_paused() {
            return;
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        self.ant_box.draw(frame);
    }

    pub fn get_height(&self) -> u32 {
        self.ant_box.get_height()
    }

    pub fn get_width(&self) -> u32 {
        self.ant_box.get_width()
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
}
