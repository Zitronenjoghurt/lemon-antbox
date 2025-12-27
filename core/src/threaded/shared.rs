use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU8, Ordering};

pub struct SharedState {
    ant_count: AtomicU16,
    is_paused: AtomicBool,
    steps_per_second: AtomicU8,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            ant_count: AtomicU16::new(0),
            is_paused: AtomicBool::new(false),
            steps_per_second: AtomicU8::new(60),
        }
    }
}

impl SharedState {
    pub fn ant_count(&self) -> u16 {
        self.ant_count.load(Ordering::Relaxed)
    }

    pub fn set_ant_count(&self, ant_count: u16) {
        self.ant_count.store(ant_count, Ordering::Relaxed);
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::Relaxed)
    }

    pub fn set_paused(&self, paused: bool) {
        self.is_paused.store(paused, Ordering::Relaxed);
    }

    pub fn steps_per_second(&self) -> u8 {
        self.steps_per_second.load(Ordering::Relaxed)
    }

    pub fn set_steps_per_second(&self, steps_per_second: u8) {
        self.steps_per_second
            .store(steps_per_second, Ordering::Relaxed);
    }
}
