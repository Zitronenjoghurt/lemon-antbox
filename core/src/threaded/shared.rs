use crate::simulation::settings::SimulationSettings;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU8, Ordering};

pub struct SharedState {
    ant_count: AtomicU16,
    is_paused: AtomicBool,
    steps_per_second: AtomicU8,
    pheromone_decay: AtomicU32,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            ant_count: AtomicU16::new(0),
            is_paused: AtomicBool::new(false),
            steps_per_second: AtomicU8::new(60),
            pheromone_decay: AtomicU32::new(0.995 as u32),
        }
    }
}

impl SharedState {
    pub fn from_settings(settings: &SimulationSettings) -> Self {
        Self {
            ant_count: AtomicU16::new(0),
            is_paused: AtomicBool::new(settings.paused),
            steps_per_second: AtomicU8::new(settings.steps_per_second),
            pheromone_decay: AtomicU32::new(settings.pheromone_decay as u32),
        }
    }

    pub fn sync_settings(&self, settings: &mut SimulationSettings) {
        settings.paused = self.is_paused.load(Ordering::Relaxed);
        settings.steps_per_second = self.steps_per_second.load(Ordering::Relaxed);
        settings.pheromone_decay = self.pheromone_decay.load(Ordering::Relaxed) as f32;
    }

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

    pub fn pheromone_decay(&self) -> f32 {
        self.pheromone_decay.load(Ordering::Relaxed) as f32
    }

    pub fn set_pheromone_decay(&self, decay: f32) {
        self.pheromone_decay.store(decay as u32, Ordering::Relaxed);
    }
}
