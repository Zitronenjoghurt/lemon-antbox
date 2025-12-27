use crate::ant_box::AntBox;

pub struct Simulation {
    ant_box: AntBox,
    paused: bool,
}

impl Simulation {
    pub fn new(height: u16, width: u16) -> Self {
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

    pub fn clear(&mut self) {
        self.ant_box.clear();
    }

    pub fn get_height(&self) -> u16 {
        self.ant_box.get_height()
    }

    pub fn get_width(&self) -> u16 {
        self.ant_box.get_width()
    }

    pub fn ant_count(&self) -> u16 {
        self.ant_box.ant_count()
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn spawn_ant(&mut self, x: u16, y: u16) {
        self.ant_box.spawn_ant(x, y);
    }
}
