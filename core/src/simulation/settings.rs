use crate::simulation::pheromones::PheromoneType;

pub struct SimulationSettings {
    pub width: u16,
    pub height: u16,
    pub tribe_count: u8,
    pub steps_per_second: u8,
    pub pheromone_decay: f32,
    pub ant_pheromone_strength: f32,
    pub ant_sensor_angle: f32,
    pub ant_sensor_distance: f32,
    pub ant_speed: f32,
    pub ant_turn_angle: f32,
    pub ant_wobble_strength: f32,
    pub paused: bool,
    pub drawn_pheromone: Option<PheromoneType>,
    pub drawn_pheromone_max_heat: f32,
    pub drawn_pheromone_tribe: u8,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            width: 640,
            height: 360,
            tribe_count: 4,
            steps_per_second: 60,
            ant_pheromone_strength: 1.0,
            pheromone_decay: 0.9975,
            ant_sensor_angle: 0.4,
            ant_sensor_distance: 10.0,
            ant_speed: 1.0,
            ant_turn_angle: 0.2,
            ant_wobble_strength: 0.4,
            drawn_pheromone: Some(PheromoneType::Home),
            drawn_pheromone_max_heat: 10.0,
            drawn_pheromone_tribe: 0,
            paused: false,
        }
    }
}

impl SimulationSettings {
    pub fn cell_count(&self) -> usize {
        self.width as usize * self.height as usize
    }
}
