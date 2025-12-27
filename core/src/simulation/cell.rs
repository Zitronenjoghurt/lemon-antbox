use bitflags::bitflags;

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {
    pub flags: CellFlags,
    pub tribe: u8,
}

impl Cell {
    pub fn color_rgba(&self) -> [u8; 4] {
        [0, 0, 0, 255]
    }
}

bitflags! {
    #[derive(Debug, Default, Copy, Clone)]
    pub struct CellFlags: u8 {
        const HAS_HOME = 0b0000_0001;
        const HAS_FOOD = 0b0000_0010;
    }
}

impl CellFlags {
    pub fn has_home(&self) -> bool {
        self.contains(CellFlags::HAS_HOME)
    }

    pub fn has_food(&self) -> bool {
        self.contains(CellFlags::HAS_FOOD)
    }

    pub fn set_home(&mut self, has_home: bool) {
        self.set(CellFlags::HAS_HOME, has_home);
    }

    pub fn set_food(&mut self, has_food: bool) {
        self.set(CellFlags::HAS_FOOD, has_food);
    }
}
