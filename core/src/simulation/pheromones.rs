use crate::simulation::ant::Ant;
use rayon::prelude::*;

const PHEROMONE_COUNT: usize = 2;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PheromoneType {
    Home = 0,
    Food = 1,
}

pub struct Pheromones {
    layers: Vec<Vec<f32>>,
    width: u16,
    height: u16,
}

impl Pheromones {
    pub fn new(width: u16, height: u16, tribe_count: u8) -> Self {
        let layer_count = tribe_count as usize * PHEROMONE_COUNT;
        let cell_count = width as usize * height as usize;
        let layers = vec![vec![0.0; cell_count]; layer_count];
        Self {
            layers,
            width,
            height,
        }
    }

    fn layer_index(&self, tribe: u8, pheromone: PheromoneType) -> usize {
        (tribe as usize * PHEROMONE_COUNT) + pheromone as usize
    }

    fn grid_index(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }

    pub fn get(&self, tribe: u8, pheromone: PheromoneType, x: u16, y: u16) -> f32 {
        self.layers[self.layer_index(tribe, pheromone)][self.grid_index(x, y)]
    }

    pub fn put(&mut self, tribe: u8, pheromone_type: PheromoneType, x: u16, y: u16, value: f32) {
        let layer_index = self.layer_index(tribe, pheromone_type);
        let grid_index = self.grid_index(x, y);
        self.layers[layer_index][grid_index] = value;
    }

    pub fn deposit(&mut self, ant: &Ant, pheromone_type: PheromoneType, value: f32) {
        self.put(ant.tribe, pheromone_type, ant.x as u16, ant.y as u16, value);
    }

    pub fn decay(&mut self, decay_factor: f32) {
        self.layers.par_iter_mut().for_each(|layer| {
            layer.iter_mut().for_each(|value| {
                *value *= decay_factor;
                if *value < 0.001 {
                    *value = 0.0;
                }
            });
        })
    }

    pub fn tribe_count(&self) -> u8 {
        (self.layers.len() / PHEROMONE_COUNT) as u8
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }
}
