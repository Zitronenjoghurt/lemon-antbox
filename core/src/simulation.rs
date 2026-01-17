use crate::simulation::ant::{Ant, AntAction, AntFeedback, AntSenses};
use crate::simulation::cell::Cell;
use crate::simulation::pheromones::{PheromoneType, Pheromones};
use crate::simulation::settings::SimulationSettings;
use crate::simulation::stats::SimulationStats;
use crate::utils::color::alpha_blend;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::time::Instant;

pub mod ant;
pub mod cell;
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

    pub fn settings(&self) -> &SimulationSettings {
        &self.settings
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

    fn index_to_coords(&self, index: usize) -> (u16, u16) {
        let x = (index % self.settings.width as usize) as u16;
        let y = (index / self.settings.width as usize) as u16;
        (x, y)
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

    pub fn get_cell(&self, x: u16, y: u16) -> Option<Cell> {
        let index = self.coords_to_index(x, y);
        self.cells.get(index).copied()
    }

    pub fn get_ant(&self, index: usize) -> Option<Ant> {
        self.ants.get(index).cloned()
    }

    pub fn get_ant_index_at_coords(&self, x: u16, y: u16, radius: f32) -> Option<usize> {
        self.ants
            .iter()
            .enumerate()
            .filter_map(|(i, a)| {
                let dist_sq = (a.x - x as f32).powi(2) + (a.y - y as f32).powi(2);
                if dist_sq <= radius * radius {
                    Some((i, dist_sq))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(i, _)| i)
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
        for (i, ant) in self.ants.iter().enumerate() {
            let index = self.coords_to_index(ant.x as u16, ant.y as u16) * 4;
            let inspected = self.settings.inspected_ant == Some(i as u16);
            frame[index..index + 4].copy_from_slice(&ant.color_rgba(inspected));
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

        let ant_actions = self
            .ants
            .par_iter()
            .map(|ant| ant.sense(self.sense_for_ant(ant), &self.settings.ant))
            .collect::<Vec<_>>();

        for (ant, action) in self.ants.iter_mut().zip(ant_actions.into_iter()) {
            Self::apply_action(
                ant,
                action,
                &self.settings,
                &mut self.pheromones,
                &mut self.cells,
            );
        }

        for x in 0..self.settings.width {
            for y in 0..self.settings.height {
                let cell_index = self.coords_to_index(x, y);
                let cell = &self.cells[cell_index];
                if cell.flags.has_home() {
                    self.pheromones.put(
                        cell.tribe,
                        PheromoneType::Home,
                        x,
                        y,
                        self.settings.nest_pheromone_strength,
                    );
                }
            }
        }

        self.pheromones.decay(self.settings.pheromone_decay);
        self.pheromones.diffuse(self.settings.pheromone_diffusion);

        self.collect_stats(start);
    }

    pub fn sense_for_ant(&self, ant: &Ant) -> AntSenses {
        let pheromone = ant.desired_pheromone();

        let left = if let Some(pheromone) = pheromone {
            self.sample_pheromone(
                ant,
                ant.angle - self.settings.ant.sensor_angle,
                self.settings.ant.sensor_distance,
                pheromone,
            )
        } else {
            0.0
        };

        let forward = if let Some(pheromone) = pheromone {
            self.sample_pheromone(ant, ant.angle, self.settings.ant.sensor_distance, pheromone)
        } else {
            0.0
        };

        let right = if let Some(pheromone) = pheromone {
            self.sample_pheromone(
                ant,
                ant.angle + self.settings.ant.sensor_angle,
                self.settings.ant.sensor_distance,
                pheromone,
            )
        } else {
            0.0
        };

        let cell_index = self.coords_to_index(ant.x as u16, ant.y as u16);
        let (food, at_home) = if let Some(cell) = self.cells.get(cell_index) {
            (cell.food, cell.flags.has_home() && cell.tribe == ant.tribe)
        } else {
            (0, false)
        };

        AntSenses {
            left,
            forward,
            right,
            food,
            at_home,
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

    fn apply_action(
        ant: &mut Ant,
        action: AntAction,
        settings: &SimulationSettings,
        pheromones: &mut Pheromones,
        cells: &mut [Cell],
    ) {
        let cell_idx = ant.y as usize * settings.width as usize + ant.x as usize;
        let cell = &mut cells[cell_idx];

        let picked_up_food = if action.pickup_food && cell.food > 0 {
            cell.food -= 1;
            true
        } else {
            false
        };
        let deposited_food = action.deposit_food;

        if let Some(pheromone) = action.deposit_pheromone {
            pheromones.deposit(ant, pheromone, action.deposit_pheromone_strength)
        }

        let feedback = AntFeedback {
            turn: action.turn,
            picked_up_food,
            deposited_food,
        };

        ant.update(&feedback, &settings.ant);

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
