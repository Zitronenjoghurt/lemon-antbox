use crate::simulation::pheromones::PheromoneType;
use crate::simulation::settings::SimulationSettings;
use crate::simulation::stats::SimulationStats;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};

pub struct SharedState {
    // Stats
    ant_count: AtomicU16,
    ants_with_food: AtomicU16,
    total_food: AtomicU64,
    avg_step_duration_secs: AtomicU32,
    // Settings
    is_paused: AtomicBool,
    steps_per_second: AtomicU8,
    pheromone_decay: AtomicU32,
    drawn_pheromone: AtomicU8,
    drawn_pheromone_tribe: AtomicU8,
    tribe_count: AtomicU8,
}

impl SharedState {
    pub fn from_settings(settings: &SimulationSettings) -> Self {
        Self {
            ant_count: AtomicU16::new(0),
            ants_with_food: AtomicU16::new(0),
            total_food: AtomicU64::new(0),
            avg_step_duration_secs: AtomicU32::new(0),
            is_paused: AtomicBool::new(settings.paused),
            steps_per_second: AtomicU8::new(settings.steps_per_second),
            pheromone_decay: AtomicU32::new(settings.pheromone_decay.to_bits()),
            drawn_pheromone: AtomicU8::new(
                settings.drawn_pheromone.map(|p| p as u8).unwrap_or(255),
            ),
            drawn_pheromone_tribe: AtomicU8::new(settings.drawn_pheromone_tribe),
            tribe_count: AtomicU8::new(settings.tribe_count),
        }
    }

    pub fn sync_settings(&self, settings: &mut SimulationSettings) {
        settings.paused = self.is_paused.load(Ordering::Relaxed);
        settings.steps_per_second = self.steps_per_second.load(Ordering::Relaxed);
        settings.pheromone_decay = f32::from_bits(self.pheromone_decay.load(Ordering::Relaxed));
        settings.drawn_pheromone = self.drawn_pheromone.load(Ordering::Relaxed).try_into().ok();
        settings.drawn_pheromone_tribe = self.drawn_pheromone_tribe.load(Ordering::Relaxed);
    }

    pub fn sync_stats(&self, stats: &SimulationStats) {
        self.set_ant_count(stats.ant_count);
        self.set_ants_with_food(stats.ants_with_food);
        self.set_total_food(stats.total_food);
        self.set_avg_step_duration_secs(stats.avg_step_duration_secs);
    }

    pub fn ant_count(&self) -> u16 {
        self.ant_count.load(Ordering::Relaxed)
    }

    pub fn set_ant_count(&self, ant_count: u16) {
        self.ant_count.store(ant_count, Ordering::Relaxed);
    }

    pub fn ants_with_food(&self) -> u16 {
        self.ants_with_food.load(Ordering::Relaxed)
    }

    pub fn set_ants_with_food(&self, ants_with_food: u16) {
        self.ants_with_food.store(ants_with_food, Ordering::Relaxed);
    }

    pub fn total_food(&self) -> u64 {
        self.total_food.load(Ordering::Relaxed)
    }

    pub fn set_total_food(&self, total_food: u64) {
        self.total_food.store(total_food, Ordering::Relaxed);
    }

    pub fn avg_step_duration_secs(&self) -> f32 {
        f32::from_bits(self.avg_step_duration_secs.load(Ordering::Relaxed))
    }

    pub fn set_avg_step_duration_secs(&self, avg_step_duration_secs: f32) {
        self.avg_step_duration_secs
            .store(avg_step_duration_secs.to_bits(), Ordering::Relaxed);
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
        f32::from_bits(self.pheromone_decay.load(Ordering::Relaxed))
    }

    pub fn set_pheromone_decay(&self, decay: f32) {
        self.pheromone_decay
            .store(decay.to_bits(), Ordering::Relaxed);
    }

    pub fn drawn_pheromone(&self) -> Option<PheromoneType> {
        self.drawn_pheromone.load(Ordering::Relaxed).try_into().ok()
    }

    pub fn set_drawn_pheromone(&self, pheromone: Option<PheromoneType>) {
        self.drawn_pheromone
            .store(pheromone.map(|p| p as u8).unwrap_or(255), Ordering::Relaxed);
    }

    pub fn drawn_pheromone_tribe(&self) -> u8 {
        self.drawn_pheromone_tribe.load(Ordering::Relaxed)
    }

    pub fn set_drawn_pheromone_tribe(&self, tribe: u8) {
        self.drawn_pheromone_tribe.store(tribe, Ordering::Relaxed);
    }

    pub fn tribe_count(&self) -> u8 {
        self.tribe_count.load(Ordering::Relaxed)
    }
}
