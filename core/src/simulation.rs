use crate::simulation::ant::Ant;
use crate::simulation::cell::Cell;
use crate::simulation::pheromones::{PheromoneType, Pheromones};
use crate::simulation::settings::SimulationSettings;
use crate::simulation::stats::SimulationStats;
use crate::utils::color::alpha_blend;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::time::Instant;

mod ant;
mod cell;
pub mod pheromones;
pub mod settings;
pub mod stats;

pub struct Simulation {
    ants: Vec<Ant>,
    cells: Vec<Cell>,
    pheromones: Pheromones,
    settings: SimulationSettings,
    stats: SimulationStats,
}

impl Simulation {
    pub fn new(settings: SimulationSettings) -> Self {
        let cells = vec![Cell::default(); settings.cell_count()];
        Self {
            ants: Vec::new(),
            cells,
            pheromones: Pheromones::new(settings.width, settings.height, settings.tribe_count),
            settings,
            stats: SimulationStats::default(),
        }
    }

    pub fn clear(&mut self) {
        self.ants.clear();
        self.cells = vec![Cell::default(); self.settings.cell_count()];
        self.pheromones.clear();
    }

    pub fn settings_mut(&mut self) -> &mut SimulationSettings {
        &mut self.settings
    }

    pub fn stats(&self) -> &SimulationStats {
        &self.stats
    }

    pub fn ant_count(&self) -> u16 {
        self.ants.len() as u16
    }

    fn coords_to_index(&self, x: u16, y: u16) -> usize {
        y as usize * self.settings.width as usize + x as usize
    }

    pub fn spawn_ant(&mut self, x: u16, y: u16, tribe: u8) {
        if x >= self.settings.width
            || y >= self.settings.height
            || self.ants.len() > 65535
            || tribe >= self.settings.tribe_count
        {
            return;
        }

        let ant = Ant {
            x: x as f32,
            y: y as f32,
            angle: fastrand::f32() * std::f32::consts::PI * 2.0,
            tribe,
            ..Default::default()
        };
        self.ants.push(ant);
    }

    pub fn spawn_nest(&mut self, x: u16, y: u16, tribe: u8) {
        if x >= self.settings.width
            || y >= self.settings.height
            || tribe >= self.settings.tribe_count
        {
            return;
        }

        let index = self.coords_to_index(x, y);
        self.cells[index].tribe = tribe;
        self.cells[index].flags.set_home(true);
    }

    pub fn spawn_food(&mut self, x: u16, y: u16, amount: u8) {
        if x >= self.settings.width || y >= self.settings.height {
            return;
        }

        let index = self.coords_to_index(x, y);
        self.cells[index].food = self.cells[index].food.saturating_add(amount);
    }
}

// Draw
impl Simulation {
    pub fn draw(&self, frame: &mut [u8]) {
        self.draw_cells(frame);
        self.draw_pheromones(frame);
        self.draw_ants(frame);
    }

    fn draw_ants(&self, frame: &mut [u8]) {
        for ant in &self.ants {
            let index = self.coords_to_index(ant.x as u16, ant.y as u16) * 4;
            frame[index..index + 4].copy_from_slice(&ant.color_rgba());
        }
    }

    fn draw_pheromones(&self, frame: &mut [u8]) {
        if self.settings.drawn_pheromone_tribe >= self.settings.tribe_count {
            return;
        }

        let Some(pheromone) = self.settings.drawn_pheromone else {
            return;
        };

        for (value, pixel) in self
            .pheromones
            .get_layer(self.settings.drawn_pheromone_tribe, pheromone)
            .iter()
            .zip(frame.chunks_exact_mut(4))
        {
            let alpha = ((*value / self.settings.drawn_pheromone_max_heat) * 255.0) as u8;
            let color = [255, 0, 0, alpha];
            let final_color = alpha_blend(color, pixel.try_into().unwrap());
            pixel.copy_from_slice(&final_color);
        }
    }

    fn draw_cells(&self, frame: &mut [u8]) {
        for (cell, pixel) in self.cells.iter().zip(frame.chunks_exact_mut(4)) {
            pixel.copy_from_slice(&cell.color_rgba());
        }
    }
}

// Step
impl Simulation {
    pub fn step(&mut self) {
        let start = Instant::now();

        if self.settings.paused {
            return;
        }

        let ant_moves = self
            .ants
            .par_iter()
            .map(|ant| self.compute_ant_move(ant))
            .collect::<Vec<_>>();

        for (ant, ant_move) in self.ants.iter_mut().zip(ant_moves.iter()) {
            Self::apply_move(
                ant,
                ant_move,
                &self.settings,
                &mut self.pheromones,
                &mut self.cells,
            );
        }

        self.pheromones.decay(self.settings.pheromone_decay);
        self.pheromones.diffuse(self.settings.pheromone_diffusion);

        self.collect_stats(start);
    }

    fn compute_ant_move(&self, ant: &Ant) -> AntMove {
        let target_pheromone = if ant.has_food {
            PheromoneType::Home
        } else {
            PheromoneType::Food
        };

        let left = self.sample_pheromone(
            ant,
            ant.angle - self.settings.ant_sensor_angle,
            self.settings.ant_sensor_distance,
            target_pheromone,
        );
        let forward = self.sample_pheromone(
            ant,
            ant.angle,
            self.settings.ant_sensor_distance,
            target_pheromone,
        );
        let right = self.sample_pheromone(
            ant,
            ant.angle + self.settings.ant_sensor_angle,
            self.settings.ant_sensor_distance,
            target_pheromone,
        );

        let turn = if forward >= left && forward >= right {
            0.0
        } else if left > right {
            -self.settings.ant_turn_angle
        } else {
            self.settings.ant_turn_angle
        };

        let wobble = (fastrand::f32() - 0.5) * self.settings.ant_wobble_strength;

        AntMove {
            turn: turn + wobble,
        }
    }

    fn sample_pheromone(
        &self,
        ant: &Ant,
        angle: f32,
        dist: f32,
        pheromone_type: PheromoneType,
    ) -> f32 {
        let sx = ant.x + angle.cos() * dist;
        let sy = ant.y + angle.sin() * dist;

        if sx < 0.0
            || sy < 0.0
            || sx >= self.settings.width as f32
            || sy >= self.settings.height as f32
        {
            return 0.0;
        }

        self.pheromones
            .get(ant.tribe, pheromone_type, sx as u16, sy as u16)
    }

    fn apply_move(
        ant: &mut Ant,
        ant_move: &AntMove,
        settings: &SimulationSettings,
        pheromones: &mut Pheromones,
        cells: &mut [Cell],
    ) {
        ant.angle += ant_move.turn;

        ant.x += ant.angle.cos() * settings.ant_speed;
        ant.y += ant.angle.sin() * settings.ant_speed;

        // Bounce off walls
        if ant.x < 0.0 {
            ant.x = -ant.x;
            ant.angle = std::f32::consts::PI - ant.angle;
        } else if ant.x >= settings.width as f32 {
            ant.x = 2.0 * settings.width as f32 - ant.x - 1.0;
            ant.angle = std::f32::consts::PI - ant.angle;
        }

        if ant.y < 0.0 {
            ant.y = -ant.y;
            ant.angle = -ant.angle;
        } else if ant.y >= settings.height as f32 {
            ant.y = 2.0 * settings.height as f32 - ant.y - 1.0;
            ant.angle = -ant.angle;
        }

        let cell_idx = ant.y as usize * settings.width as usize + ant.x as usize;
        let cell = &mut cells[cell_idx];

        if !ant.has_food && cell.food > 0 {
            ant.has_food = true;
            cell.food -= 1;
            ant.angle += std::f32::consts::PI;
        }

        if ant.has_food && cell.flags.has_home() && cell.tribe == ant.tribe {
            ant.has_food = false;
            ant.angle += std::f32::consts::PI;
        }

        let deposit_pheromone = if ant.has_food {
            PheromoneType::Food
        } else {
            PheromoneType::Home
        };
        pheromones.deposit(ant, deposit_pheromone, settings.ant_pheromone_strength);
    }

    fn collect_stats(&mut self, instant_start: Instant) {
        self.stats.ant_count = self.ants.len() as u16;
        self.stats.ants_with_food = self.ants.par_iter().filter(|a| a.has_food).count() as u16;
        self.stats.total_food = self.cells.par_iter().map(|c| c.food as u64).sum();

        let duration = instant_start.elapsed().as_secs_f32();
        const SMOOTHING: f32 = 0.05;
        self.stats.avg_step_duration_secs =
            self.stats.avg_step_duration_secs * (1.0 - SMOOTHING) + duration * SMOOTHING;
    }
}

struct AntMove {
    turn: f32,
}
