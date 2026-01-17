use crate::simulation::pheromones::PheromoneType;

pub struct AntSettings {
    pub pheromone_strength: f32,
    pub sensor_angle: f32,
    pub sensor_distance: f32,
    pub speed: f32,
    pub turn_angle: f32,
    pub wobble_strength: f32,
    pub spiral_expansion_rate: f32,
}

impl Default for AntSettings {
    fn default() -> Self {
        Self {
            pheromone_strength: 1.0,
            sensor_angle: 0.4,
            sensor_distance: 10.0,
            speed: 1.0,
            turn_angle: 0.2,
            wobble_strength: 0.4,
            spiral_expansion_rate: 0.01,
        }
    }
}

pub struct SimulationSettings {
    pub width: u16,
    pub height: u16,
    pub ant: AntSettings,
    pub tribe_count: u8,
    pub steps_per_second: u8,
    pub pheromone_decay: f32,
    pub pheromone_diffusion: f32,
    pub nest_pheromone_strength: f32,
    pub paused: bool,
    pub drawn_pheromone: Option<PheromoneType>,
    pub drawn_pheromone_max_heat: f32,
    pub drawn_pheromone_tribe: u8,
    pub inspected_ant: Option<u16>,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            width: 640,
            height: 360,
            ant: AntSettings::default(),
            tribe_count: 4,
            steps_per_second: 60,
            pheromone_decay: 0.9975,
            pheromone_diffusion: 0.25,
            nest_pheromone_strength: 5.0,
            drawn_pheromone: Some(PheromoneType::Home),
            drawn_pheromone_max_heat: 10.0,
            drawn_pheromone_tribe: 0,
            paused: false,
            inspected_ant: None,
        }
    }
}

impl SimulationSettings {
    pub fn cell_count(&self) -> usize {
        self.width as usize * self.height as usize
    }
}
