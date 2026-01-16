use bitflags::bitflags;

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {
    pub flags: CellFlags,
    pub tribe: u8,
    pub food: u8,
}

impl Cell {
    pub fn color_rgba(&self) -> [u8; 4] {
        if self.flags.has_home() {
            [0, 0, 255, 255]
        } else {
            [0, self.food, 0, 255]
        }
    }
}

bitflags! {
    #[derive(Debug, Default, Copy, Clone)]
    pub struct CellFlags: u8 {
        const HAS_HOME = 0b0000_0001;
    }
}

impl CellFlags {
    pub fn has_home(&self) -> bool {
        self.contains(CellFlags::HAS_HOME)
    }

    pub fn set_home(&mut self, has_home: bool) {
        self.set(CellFlags::HAS_HOME, has_home);
    }
}
